# README

This documentation aims to set up a working RUST/Rocket project environment.

*Notice: Sometimes you might get an error when building the image. This could be caused by existing `target` files, previously 
created by the Docker root user, and not being editable by your current user. One way to solve this would be to add the 
ACL to the current folder.*

```bash
# optional
sudo setfacl -R -m u:"$(whoami)":rwX $(pwd)
```

## Dev

Howto get the [dev environment](./README_dev.md) ready


Done!
