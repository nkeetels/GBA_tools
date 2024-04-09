# GBA_tools
Tools for streamlining Gameboy Advance game development

obj_converter: converts Wavefront OBJ to const array of 16-bit signed vertex components (positions, texture coordinates, normals). Make sure to either export texture coordinates or normals, if both are present then normals take precedense over texture coordinates (x, y, z, u, v, w).
Usage: obj_converter.exe filename.obj
