use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::sky::buffers::SkyBuffers;
use crate::types::SkyMode;
use std::error::Error;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Device};

pub struct SkyBindGroups {
    sky_bind_group: BindGroup,
    sun_bind_group: BindGroup,
    moon_bind_group: Option<BindGroup>,
    star_map_bind_group: Option<BindGroup>,
    cloud_bind_group: Option<BindGroup>,
}

impl SkyBindGroups {
    pub fn new(
        use_pbr: bool,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> Self {
        let sky_bind_group = if use_pbr {
            device.create_bind_group(&BindGroupDescriptor {
                label: Some("sky_pbr"),
                layout: layouts.sky_pbr(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: buffers.sky_pbr_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: buffers.sun_ubo.as_entire_binding(),
                    },
                ],
            })
        } else {
            device.create_bind_group(&BindGroupDescriptor {
                label: Some("sky_gradient"),
                layout: layouts.sky_gradient(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: buffers.sky_gradient_ubo.as_entire_binding(),
                }],
            })
        };

        let sun_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("sun"),
            layout: layouts.sun(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffers.sun_ubo.as_entire_binding(),
            }],
        });

        Self {
            sky_bind_group,
            sun_bind_group,
            moon_bind_group: None,
            star_map_bind_group: None,
            cloud_bind_group: None,
        }
    }

    pub fn set_sky_mode(
        &mut self,
        mode: SkyMode,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) -> Result<(), Box<dyn Error>> {
        match mode {
            SkyMode::None => Ok(()),
            SkyMode::Gradient => {
                self.sky_bind_group = device.create_bind_group(&BindGroupDescriptor {
                    label: Some("sky_gradient"),
                    layout: layouts.sky_gradient(),
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: buffers.sky_gradient_ubo.as_entire_binding(),
                    }],
                });
                Ok(())
            }
            SkyMode::Pbr => {
                self.sky_bind_group = device.create_bind_group(&BindGroupDescriptor {
                    label: Some("sky_pbr"),
                    layout: layouts.sky_pbr(),
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: buffers.sky_pbr_ubo.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: buffers.sun_ubo.as_entire_binding(),
                        },
                    ],
                });
                Ok(())
            }
            SkyMode::Texture => {
                if let Some(sky_box_texture) = &buffers.sky_box_texture {
                    self.sky_bind_group = device.create_bind_group(&BindGroupDescriptor {
                        label: Some("sky_box"),
                        layout: layouts.sky_box(),
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: BindingResource::Sampler(&buffers.clamp_sampler),
                            },
                            BindGroupEntry {
                                binding: 1,
                                resource: BindingResource::TextureView(sky_box_texture),
                            },
                        ],
                    });
                    Ok(())
                } else {
                    Err("SkyRenderManager: could not find texture for sky box.".into())
                }
            }
        }
    }

    pub fn update_sky_box_texture(
        &mut self,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) {
        if let Some(sky_box_texture) = &buffers.sky_box_texture {
            self.sky_bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("sky_box"),
                layout: layouts.sky_box(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::Sampler(&buffers.clamp_sampler),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(sky_box_texture),
                    },
                ],
            });
        } else {
            self.sky_bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("sky_gradient"),
                layout: layouts.sky_gradient(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: buffers.sky_gradient_ubo.as_entire_binding(),
                }],
            });
        }
    }

    pub fn update_moon_textures(
        &mut self,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) {
        if let Some(moon_texture_array) = &buffers.moon_texture_array {
            self.moon_bind_group = Some(device.create_bind_group(&BindGroupDescriptor {
                label: Some("moon"),
                layout: layouts.moon(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: buffers.moon_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: buffers.sun_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::Sampler(&buffers.clamp_sampler),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::TextureView(moon_texture_array),
                    },
                ],
            }));
        } else {
            self.moon_bind_group = None;
        }
    }

    pub fn update_star_map_texture(
        &mut self,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) {
        if let Some(star_map_texture) = &buffers.star_map_texture {
            self.star_map_bind_group = Some(device.create_bind_group(&BindGroupDescriptor {
                label: Some("star_map"),
                layout: layouts.star_map(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: buffers.star_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&buffers.clamp_sampler),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(star_map_texture),
                    },
                ],
            }));
        } else {
            self.star_map_bind_group = None;
        }
    }

    pub fn update_cloud_texture(
        &mut self,
        buffers: &SkyBuffers,
        layouts: &BindGroupLayouts,
        device: &Device,
    ) {
        if let Some(cloud_texture) = &buffers.cloud_texture {
            self.cloud_bind_group = Some(device.create_bind_group(&BindGroupDescriptor {
                label: Some("cloud"),
                layout: layouts.cloud(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: buffers.cloud_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: buffers.sun_ubo.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::Sampler(&buffers.tile_sampler),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::TextureView(cloud_texture),
                    },
                ],
            }));
        } else {
            self.cloud_bind_group = None;
        }
    }

    pub fn sky_bind_group(&self) -> &BindGroup {
        &self.sky_bind_group
    }

    pub fn sun_bind_group(&self) -> &BindGroup {
        &self.sun_bind_group
    }

    pub fn moon_bind_group(&self) -> Option<&BindGroup> {
        self.moon_bind_group.as_ref()
    }

    pub fn star_map_bind_group(&self) -> Option<&BindGroup> {
        self.star_map_bind_group.as_ref()
    }

    pub fn cloud_bind_group(&self) -> Option<&BindGroup> {
        self.cloud_bind_group.as_ref()
    }
}
