use crate::render_resources::RenderSettings;
use crate::render_resources::layouts::BindGroupLayouts;
use crate::render_resources::shaders::Shaders;
use crate::utils::pipeline::PipelineBuilder;
use wgpu::{
    ComputePipeline, ComputePipelineDescriptor, Device, PipelineLayout, PipelineLayoutDescriptor,
    RenderPipeline, TextureFormat,
};

pub struct PostProcessPipelines {
    adaptation_pipeline_layout: PipelineLayout,
    bloom_composite_pipeline_layout: PipelineLayout,
    bloom_extract_pipeline_layout: PipelineLayout,
    blur_pipeline_layout: PipelineLayout,
    luminance_downsample_pipeline_layout: PipelineLayout,
    tone_map_pipeline_layout: PipelineLayout,

    adaptation_pipeline: ComputePipeline,
    bloom_composite_pipeline: RenderPipeline,
    bloom_extract_pipeline: RenderPipeline,
    blur_pipeline: RenderPipeline,
    luminance_downsample_pipeline: ComputePipeline,
    tone_map_pipeline: RenderPipeline,
}

impl PostProcessPipelines {
    pub fn new(
        settings: &RenderSettings,
        layouts: &BindGroupLayouts,
        shaders: &Shaders,
        device: &Device,
    ) -> Self {
        let adaptation_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("adaptation"),
            bind_group_layouts: &[layouts.adaptation(), layouts.luminance_sample()],
            push_constant_ranges: &[],
        });

        let bloom_composite_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("bloom_composite"),
                bind_group_layouts: &[layouts.lens_flare(), layouts.blit(), layouts.blit()],
                push_constant_ranges: &[],
            });

        let bloom_extract_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("bloom_extract"),
                bind_group_layouts: &[layouts.bloom(), layouts.blit()],
                push_constant_ranges: &[],
            });

        let blur_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("blur"),
            bind_group_layouts: &[layouts.blur(), layouts.blit()],
            push_constant_ranges: &[],
        });

        let luminance_downsample_pipeline_layout =
            device.create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("luminance_downsample"),
                bind_group_layouts: &[layouts.luminance_downsample()],
                push_constant_ranges: &[],
            });

        let tone_map_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("tone_map"),
            bind_group_layouts: &[layouts.tone_map(), layouts.blit(), layouts.blit()],
            push_constant_ranges: &[],
        });

        let adaptation_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("adaptation"),
            layout: Some(&adaptation_pipeline_layout),
            module: shaders.adaptation(),
            entry_point: Some("main"),
            cache: Default::default(),
            compilation_options: Default::default(),
        });

        let bloom_composite_pipeline = PipelineBuilder::new(
            "bloom_composite",
            &bloom_composite_pipeline_layout,
            shaders.bloom_composite(),
            device,
        )
        .target_format(TextureFormat::Rgba16Float)
        .no_depth()
        .build();

        let bloom_extract_pipeline = PipelineBuilder::new(
            "bloom_extract",
            &bloom_extract_pipeline_layout,
            shaders.bloom_extract(),
            device,
        )
        .target_format(TextureFormat::Rgba16Float)
        .no_depth()
        .build();

        let blur_pipeline =
            PipelineBuilder::new("blur", &blur_pipeline_layout, shaders.blur(), device)
                .target_format(TextureFormat::Rgba16Float)
                .no_depth()
                .build();

        let luminance_downsample_pipeline =
            device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("luminance_downsample"),
                layout: Some(&luminance_downsample_pipeline_layout),
                module: shaders.luminance_downsample(),
                entry_point: Some("main"),
                cache: Default::default(),
                compilation_options: Default::default(),
            });

        let tone_map_pipeline = PipelineBuilder::new(
            "tone_map",
            &tone_map_pipeline_layout,
            shaders.tone_map(),
            device,
        )
        .target_format(settings.surface_format)
        .no_depth()
        .build();

        Self {
            adaptation_pipeline_layout,
            bloom_composite_pipeline_layout,
            bloom_extract_pipeline_layout,
            blur_pipeline_layout,
            luminance_downsample_pipeline_layout,
            tone_map_pipeline_layout,
            adaptation_pipeline,
            bloom_composite_pipeline,
            bloom_extract_pipeline,
            blur_pipeline,
            luminance_downsample_pipeline,
            tone_map_pipeline,
        }
    }

    pub fn adaptation_pipeline_layout(&self) -> &PipelineLayout {
        &self.adaptation_pipeline_layout
    }

    pub fn bloom_composite_pipeline_layout(&self) -> &PipelineLayout {
        &self.bloom_composite_pipeline_layout
    }

    pub fn bloom_extract_pipeline_layout(&self) -> &PipelineLayout {
        &self.bloom_extract_pipeline_layout
    }

    pub fn blur_pipeline_layout(&self) -> &PipelineLayout {
        &self.blur_pipeline_layout
    }

    pub fn luminance_downsample_pipeline_layout(&self) -> &PipelineLayout {
        &self.luminance_downsample_pipeline_layout
    }

    pub fn tone_map_pipeline_layout(&self) -> &PipelineLayout {
        &self.tone_map_pipeline_layout
    }

    pub fn adaptation_pipeline(&self) -> &ComputePipeline {
        &self.adaptation_pipeline
    }

    pub fn bloom_composite_pipeline(&self) -> &RenderPipeline {
        &self.bloom_composite_pipeline
    }

    pub fn bloom_extract_pipeline(&self) -> &RenderPipeline {
        &self.bloom_extract_pipeline
    }

    pub fn blur_pipeline(&self) -> &RenderPipeline {
        &self.blur_pipeline
    }

    pub fn luminance_downsample_pipeline(&self) -> &ComputePipeline {
        &self.luminance_downsample_pipeline
    }

    pub fn tone_map_pipeline(&self) -> &RenderPipeline {
        &self.tone_map_pipeline
    }
}
