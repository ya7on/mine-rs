"""Export vanilla 26.3 registry listings and fully resolved network tags.

Inputs are Mojang's client archive and the official server registry report.
No downloads or Java execution are performed by this script.
"""

import argparse
import hashlib
import json
from pathlib import Path
import zipfile


CLIENT_SHA1 = "e877b6a07acd633fb3bb475002175cec036e7b87"
REPORT_SHA256 = "51f62d56f8bd0e9134ee88dd9226b64e9e56cf6a32377540d44291bf47ecdac6"
# RegistryDataLoader.SYNCHRONIZED_REGISTRIES in the official 26.3 archive.
SYNCHRONIZED = """
worldgen/biome chat_type trim_pattern trim_material wolf_variant
wolf_sound_variant pig_variant pig_sound_variant frog_variant cat_variant
cat_sound_variant cow_sound_variant cow_variant chicken_sound_variant
chicken_variant zombie_nautilus_variant painting_variant sulfur_cube_archetype
dimension_type damage_type banner_pattern enchantment jukebox_song instrument
test_environment test_instance dialog world_clock timeline decorated_pot_pattern
block_transformer worldgen/block_state_provider
""".split()


def qualify(value):
    return value if ":" in value else "minecraft:" + value


def export(client, report, output):
    if hashlib.sha1(client.read_bytes()).hexdigest() != CLIENT_SHA1:
        raise ValueError("Expected the official vanilla 26.3 client archive")
    if hashlib.sha256(report.read_bytes()).hexdigest() != REPORT_SHA256:
        raise ValueError("Expected the official vanilla 26.3 server registry report")
    static = json.loads(report.read_text())
    with zipfile.ZipFile(client) as archive:
        version = json.loads(archive.read("version.json"))
        if version["id"] != "26.3" or version["protocol_version"] != 777:
            raise ValueError("Wrong Minecraft version or protocol")
        names = archive.namelist()
        dynamic = {}
        for registry in SYNCHRONIZED:
            prefix = "data/minecraft/" + registry + "/"
            entries = sorted(
                "minecraft:" + name[len(prefix):-5]
                for name in names if name.startswith(prefix) and name.endswith(".json")
            )
            if not entries:
                raise ValueError("Missing registry data: " + registry)
            dynamic["minecraft:" + registry] = entries

        ids = {
            registry: {name: value["protocol_id"] for name, value in data["entries"].items()}
            for registry, data in static.items()
        }
        ids.update({registry: dict(zip(entries, range(len(entries))))
                    for registry, entries in dynamic.items()})
        raw_tags = {}
        # Longest prefix wins for registry names such as worldgen/biome.
        registries = sorted(ids, key=len, reverse=True)
        for name in names:
            prefix = "data/minecraft/tags/"
            if not name.startswith(prefix) or not name.endswith(".json"):
                continue
            path = name[len(prefix):-5]
            for registry in registries:
                registry_path = registry.split(":", 1)[1] + "/"
                if path.startswith(registry_path):
                    tag = "minecraft:" + path[len(registry_path):]
                    raw_tags.setdefault(registry, {})[tag] = json.loads(archive.read(name))["values"]
                    break

        resolved = {}
        active = set()

        def resolve(registry, tag):
            key = (registry, tag)
            if key in resolved:
                return resolved[key]
            if key in active:
                raise ValueError("Cyclic tag reference: " + repr(key))
            active.add(key)
            entries = set()
            for value in raw_tags[registry][tag]:
                required = True
                if isinstance(value, dict):
                    required = value.get("required", True)
                    value = value["id"]
                nested = value.startswith("#")
                name = qualify(value[1:] if nested else value)
                available = raw_tags[registry] if nested else ids[registry]
                if name not in available:
                    if required:
                        raise ValueError(f"Missing required reference {registry}/{tag}: {value}")
                    continue
                entries.update(resolve(registry, name) if nested else [ids[registry][name]])
            active.remove(key)
            resolved[key] = sorted(entries)
            return resolved[key]

        tags = {registry: {tag: resolve(registry, tag) for tag in sorted(values)}
                for registry, values in sorted(raw_tags.items())}

    output.mkdir(parents=True, exist_ok=True)
    # One registry/tag per line keeps generated changes reviewable without
    # expanding numeric membership lists into thousands of separate lines.
    lines = ["  " + json.dumps(key) + ": " + json.dumps(value, separators=(",", ":"))
             for key, value in sorted(dynamic.items())]
    (output / "registries-777.json").write_text("{\n" + ",\n".join(lines) + "\n}\n")
    sections = []
    for registry, values in tags.items():
        lines = ["    " + json.dumps(tag) + ": " + json.dumps(entries, separators=(",", ":"))
                 for tag, entries in values.items()]
        sections.append("  " + json.dumps(registry) + ": {\n" + ",\n".join(lines) + "\n  }")
    (output / "tags-777.json").write_text("{\n" + ",\n".join(sections) + "\n}\n")
    print(f"Exported {len(dynamic)} registries, {sum(map(len, dynamic.values()))} entries; "
          f"{len(tags)} tag registries, {sum(map(len, tags.values()))} tags")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--client", type=Path, required=True)
    parser.add_argument("--registry-report", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    export(args.client, args.registry_report, args.output)
