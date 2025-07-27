use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::shaders::Shaders;
use crate::utils::pipeline::PipelineBuilder;
use wgpu::BlendFactor::One;
use wgpu::BlendOperation::Add;
use wgpu::{
    BlendComponent, BlendState, Device, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline,
};

pub struct SkyPipelines {
    sky_box_pipeline_layout: PipelineLayout,
    sky_gradient_pipeline_layout: PipelineLayout,
    sky_pbr_pipeline_layout: PipelineLayout,
    sun_pipeline_layout: PipelineLayout,
    moon_pipeline_layout: PipelineLayout,
    star_map_pipeline_layout: PipelineLayout,
    cloud_pipeline_layout: PipelineLayout,

    sky_box_pipeline: RenderPipeline,
    sky_gradient_pipeline: RenderPipeline,
    sky_pbr_pipeline: RenderPipeline,
    sun_pipeline: RenderPipeline,
    moon_pipeline: RenderPipeline,
    star_map_pipeline: RenderPipeline,
    cloud_pipeline: RenderPipeline,
}

impl SkyPipelines {
    pub fn new(layouts: &BindGroupLayouts, shaders: &Shaders, device: &Device) -> Self {
        let sky_box_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("sky_box"),
            bind_group_layouts: &[layouts.camera_view(), layouts.sky_box()],
            push_constant_ranges: &[],
        });

        let sky_gradient_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("sky_gradient"),
                bind_group_layouts: &[layouts.camera_view(), layouts.sky_gradient()],
                push_constant_ranges: &[],
            });

        let sky_pbr_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("sky_pbr"),
            bind_group_layouts: &[layouts.camera_view(), layouts.sky_pbr()],
            push_constant_ranges: &[],
        });

        let sun_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("sun"),
            bind_group_layouts: &[layouts.camera_view(), layouts.sun()],
            push_constant_ranges: &[],
        });

        let moon_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("moon"),
            bind_group_layouts: &[layouts.camera_view(), layouts.moon()],
            push_constant_ranges: &[],
        });

        let star_map_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("star_map"),
            bind_group_layouts: &[layouts.camera_view(), layouts.star_map()],
            push_constant_ranges: &[],
        });

        let cloud_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("cloud"),
            bind_group_layouts: &[layouts.camera_view(), layouts.cloud()],
            push_constant_ranges: &[],
        });

        let sky_box_pipeline = PipelineBuilder::new(
            "sky_box",
            &sky_box_pipeline_layout,
            shaders.sky_box(),
            device,
        )
        .no_depth_write()
        .build();

        let sky_gradient_pipeline = PipelineBuilder::new(
            "sky_gradient",
            &sky_gradient_pipeline_layout,
            shaders.sky_gradient(),
            device,
        )
        .no_depth_write()
        .build();

        let sky_pbr_pipeline = PipelineBuilder::new(
            "sky_pbr",
            &sky_pbr_pipeline_layout,
            shaders.sky_pbr(),
            device,
        )
        .blend_state(BlendState {
            color: BlendComponent {
                src_factor: One,
                dst_factor: One,
                operation: Add,
            },
            alpha: BlendComponent::REPLACE,
        })
        .no_depth_write()
        .build();

        let sun_pipeline = PipelineBuilder::new("sun", &sun_pipeline_layout, shaders.sun(), device)
            .blend_state(BlendState::ALPHA_BLENDING)
            .no_depth_write()
            .build();

        let moon_pipeline =
            PipelineBuilder::new("moon", &moon_pipeline_layout, shaders.moon(), device)
                .blend_state(BlendState::ALPHA_BLENDING)
                .no_depth_write()
                .build();

        let star_map_pipeline = PipelineBuilder::new(
            "star_map",
            &star_map_pipeline_layout,
            shaders.star_map(),
            device,
        )
        .blend_state(BlendState::ALPHA_BLENDING)
        .no_depth_write()
        .build();

        let cloud_pipeline =
            PipelineBuilder::new("cloud", &cloud_pipeline_layout, shaders.cloud(), device)
                .blend_state(BlendState::ALPHA_BLENDING)
                .no_depth_write()
                .build();

        Self {
            sky_box_pipeline_layout,
            sky_gradient_pipeline_layout,
            sky_pbr_pipeline_layout,
            sun_pipeline_layout,
            moon_pipeline_layout,
            star_map_pipeline_layout,
            cloud_pipeline_layout,
            sky_box_pipeline,
            sky_gradient_pipeline,
            sky_pbr_pipeline,
            sun_pipeline,
            moon_pipeline,
            star_map_pipeline,
            cloud_pipeline,
        }
    }

    pub fn sky_box_pipeline(&self) -> &RenderPipeline {
        &self.sky_box_pipeline
    }

    pub fn sky_gradient_pipeline(&self) -> &RenderPipeline {
        &self.sky_gradient_pipeline
    }

    pub fn sky_pbr_pipeline(&self) -> &RenderPipeline {
        &self.sky_pbr_pipeline
    }

    pub fn sun_pipeline(&self) -> &RenderPipeline {
        &self.sun_pipeline
    }

    pub fn moon_pipeline(&self) -> &RenderPipeline {
        &self.moon_pipeline
    }

    pub fn star_map_pipeline(&self) -> &RenderPipeline {
        &self.star_map_pipeline
    }

    pub fn cloud_pipeline(&self) -> &RenderPipeline {
        &self.cloud_pipeline
    }
}
