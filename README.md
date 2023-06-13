# rays
A raytracer built in rust.

## Todo List!   

### Refactors/Improvements
- camera needs a wrapper type for serialization
- scene (or something?) needs a 'render config' that stores output info, number of samples, etc

------------
**IDEA FOR SCENE** 
- owns a list of geometries and materials
- also contains a tree (or some other data structure) with references to each object (geom+mat)
- this should allow the scene to own the data internally while also (somehow!) allowing shared materials and geoms
- maybe and object is a material, a geometry, and a location/rotation/scale? would allow better geom sharing
- maybe we first create all the geometries and materials and give them to the scene, which returns some kind of key (via a hashmap),
   then we add objects to the scene by referring to the keys
------------

### Non-Raytracing-Related Features
- render in winit window rather than save to image
- save/load scenes
- multithreading

### Raytracing-Related Features
- lights
- triangles/meshes
- ray marching

### Extra
- random color generation
