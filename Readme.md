Learning Rust + Vulkano
====

Notes On Ubuntu:
----

To run on [Ubuntu 16.04 LTS](https://01.org/linuxgraphics/blogs/jekstrand/2016/open-source-vulkan-drivers-intel-hardware) with [Intel Driver](https://launchpad.net/~ubuntu-x-swat/+archive/ubuntu/intel-graphics-updates):

```
sudo add-apt-repository ppa:ubuntu-x-swat/intel-graphics-updates
sudo apt-get update

sudo apt-get install libvulkan1 mesa-vulkan-drivers
```

then you can try:

```
vulkaninfo
```
 
to get your current vulkan capabilities

then

```
cargo run
```

should work