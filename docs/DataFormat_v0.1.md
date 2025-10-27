# Arden Data Format v0.1

## Voxel Data (Stateful Voxel)
- MaterialID (uint16)
- Density (uint8)   // 0-255
- Flags (uint8)     // damage, stability, heat, etc.

### Octochunk Layout
- 32x32x32 array of voxels
- Header: version, checksum, sector ref

## DTO Structure
- Header (format version, object type)
- RawData (voxel block)
- Metadata: timestamps, flags, LOD cache indicators

DTO does NOT store mesh or physics.

### Versioning
MAJOR.MINOR.PATCH
Minor and patch versions must remain backward compatible.
