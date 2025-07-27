use wgpu::{
    BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState,
    Device, Face, FragmentState, FrontFace, MultisampleState, PipelineLayout, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    StencilState, TextureFormat, VertexBufferLayout, VertexState,
};

pub struct PipelineBuilder<'a> {
    label: &'a str,
    layout: &'a PipelineLayout,
    shader: &'a ShaderModule,
    device: &'a Device,
    vertex_buffers: &'a [VertexBufferLayout<'a>],
    fragment_enabled: bool,
    fragment_entry_point: String,
    target_format: TextureFormat,
    blend_state: Option<BlendState>,
    write_mask: ColorWrites,
    cull_mode: Option<Face>,
    polygon_mode: PolygonMode,
    topology: PrimitiveTopology,
    depth_enabled: bool,
    depth_format: TextureFormat,
    depth_write_enabled: bool,
    depth_compare: CompareFunction,
    depth_bias_state: DepthBiasState,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(
        label: &'a str,
        layout: &'a PipelineLayout,
        shader: &'a ShaderModule,
        device: &'a Device,
    ) -> Self {
        Self {
            label,
            layout,
            shader,
            device,
            vertex_buffers: &[],
            fragment_enabled: true,
            fragment_entry_point: String::from("fs_main"),
            target_format: TextureFormat::Rgba16Float,
            blend_state: None,
            write_mask: ColorWrites::ALL,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            topology: PrimitiveTopology::TriangleList,
            depth_enabled: true,
            depth_format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            depth_bias_state: DepthBiasState::default(),
        }
    }

    pub fn build(self) -> RenderPipeline {
        let color_target = Some(ColorTargetState {
            format: self.target_format,
            blend: self.blend_state,
            write_mask: self.write_mask,
        });

        let targets = &[color_target];

        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(self.label),
                layout: Some(self.layout),
                vertex: VertexState {
                    module: self.shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: self.vertex_buffers,
                },
                fragment: if self.fragment_enabled {
                    Some(FragmentState {
                        module: self.shader,
                        entry_point: Some(&self.fragment_entry_point),
                        compilation_options: Default::default(),
                        targets,
                    })
                } else {
                    None
                },
                primitive: PrimitiveState {
                    topology: self.topology,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: self.cull_mode,
                    unclipped_depth: false,
                    polygon_mode: self.polygon_mode,
                    conservative: false,
                },
                depth_stencil: if self.depth_enabled {
                    Some(DepthStencilState {
                        format: self.depth_format,
                        depth_write_enabled: self.depth_write_enabled,
                        depth_compare: self.depth_compare,
                        bias: self.depth_bias_state,
                        stencil: StencilState::default(),
                    })
                } else {
                    None
                },
                multisample: MultisampleState::default(),
                multiview: None,
                cache: None,
            })
    }

    pub fn vertex_buffers(mut self, buffers: &'a [VertexBufferLayout<'a>]) -> Self {
        self.vertex_buffers = buffers;
        self
    }

    pub fn no_fragment(mut self) -> Self {
        self.fragment_enabled = false;
        self
    }

    pub fn target_format(mut self, target_format: TextureFormat) -> Self {
        self.target_format = target_format;
        self
    }

    pub fn blend_state(mut self, blend_state: BlendState) -> Self {
        self.blend_state = Some(blend_state);
        self
    }

    pub fn write_mask(mut self, write_mask: ColorWrites) -> Self {
        self.write_mask = write_mask;
        self
    }

    pub fn cull_mode(mut self, cull_mode: Option<Face>) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    pub fn fragment_entry_point(mut self, entry_point: &str) -> Self {
        self.fragment_entry_point = entry_point.into();
        self
    }

    pub fn polygon_mode(mut self, polygon_mode: PolygonMode) -> Self {
        self.polygon_mode = polygon_mode;
        self
    }

    pub fn topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    pub fn no_depth(mut self) -> Self {
        self.depth_enabled = false;
        self
    }

    pub fn depth_format(mut self, depth_format: TextureFormat) -> Self {
        self.depth_format = depth_format;
        self
    }

    pub fn no_depth_write(mut self) -> Self {
        self.depth_write_enabled = false;
        self
    }

    pub fn depth_compare(mut self, depth_compare: CompareFunction) -> Self {
        self.depth_compare = depth_compare;
        self
    }

    pub fn depth_bias_state(mut self, depth_bias_state: DepthBiasState) -> Self {
        self.depth_bias_state = depth_bias_state;
        self
    }
}
