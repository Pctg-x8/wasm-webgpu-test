use js_sys::{Array, ArrayBuffer, Float32Array, Object, Reflect};
use wasm_bindgen::{convert::IntoWasmAbi, prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GPUBufferCreateDescription {
    #[wasm_bindgen(getter_with_clone)]
    pub label: Option<String>,
    #[wasm_bindgen(js_name = mappedAtCreation)]
    pub mapped_at_creation: Option<bool>,
    pub size: u32,
    pub usage: u32,
}

#[wasm_bindgen]
pub struct GPUCommandEncoderCreateDescription {
    #[wasm_bindgen(getter_with_clone)]
    pub label: Option<String>,
}

#[wasm_bindgen]
pub struct GPUCommandBufferDescriptor {
    #[wasm_bindgen(getter_with_clone)]
    pub label: Option<String>,
}

#[repr(transparent)]
pub struct GPURenderPassColorAttachment(Object);
impl GPURenderPassColorAttachment {
    pub fn new(view: &GPUTextureView) -> Self {
        let obj = Object::new();

        Reflect::set(&obj, &"view".into(), &view).unwrap();
        Reflect::set(&obj, &"loadOp".into(), &"load".into()).unwrap();
        Reflect::set(&obj, &"storeOp".into(), &"store".into()).unwrap();

        Self(obj)
    }

    pub fn resolve_to(self, view: &GPUTextureView) -> Self {
        Reflect::set(&self.0, &"resolveTarget".into(), view).unwrap();

        self
    }

    fn clear_by(self, value: &JsValue) -> Self {
        Reflect::set(&self.0, &"clearValue".into(), value).unwrap();
        Reflect::set(&self.0, &"loadOp".into(), &"clear".into()).unwrap();

        self
    }

    pub fn clear_by_array(self, values: &[f32; 4]) -> Self {
        let values: Array = values
            .into_iter()
            .map(|&x| JsValue::from_f64(x as _))
            .collect();

        self.clear_by(&values)
    }

    pub fn clear_by_object(self, value: &Object) -> Self {
        self.clear_by(value)
    }
}
impl AsRef<JsValue> for GPURenderPassColorAttachment {
    fn as_ref(&self) -> &JsValue {
        self.0.as_ref()
    }
}
impl IntoWasmAbi for GPURenderPassColorAttachment {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPURenderPassColorAttachment {
    fn describe() {
        Object::describe()
    }
}

pub struct GPURenderPassDescriptor {
    pub color_attachments: Vec<GPURenderPassColorAttachment>,
}
impl GPURenderPassDescriptor {
    pub fn into_object(self) -> Result<Object, JsValue> {
        let obj = Object::new();

        Reflect::set(
            &obj,
            &"colorAttachments".into(),
            &Array::from_iter(self.color_attachments),
        )?;

        Ok(obj)
    }
}
impl IntoWasmAbi for GPURenderPassDescriptor {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.into_object().expect("Failed to convert to wasm abi"))
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPURenderPassDescriptor {
    fn describe() {
        Object::describe()
    }
}

pub struct GPUCanvasConfiguration(Object);
impl GPUCanvasConfiguration {
    pub fn new(device: &GPUDevice) -> Self {
        let o = Object::new();

        Reflect::set(&o, &"device".into(), device).unwrap();

        Self(o)
    }

    pub fn with_format(self, format: &str) -> Self {
        Reflect::set(&self.0, &"format".into(), &format.into()).unwrap();

        self
    }
}
impl From<GPUCanvasConfiguration> for Object {
    fn from(value: GPUCanvasConfiguration) -> Self {
        value.0
    }
}
impl IntoWasmAbi for GPUCanvasConfiguration {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPUCanvasConfiguration {
    fn describe() {
        Object::describe()
    }
}

pub struct GPUShaderModuleDescriptor(Object);
impl GPUShaderModuleDescriptor {
    pub fn new(code: &str) -> Result<Self, JsValue> {
        let o = Object::new();

        Reflect::set(&o, &"code".into(), &code.into())?;

        Ok(Self(o))
    }
}
impl From<GPUShaderModuleDescriptor> for Object {
    fn from(value: GPUShaderModuleDescriptor) -> Self {
        value.0
    }
}
impl IntoWasmAbi for GPUShaderModuleDescriptor {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPUShaderModuleDescriptor {
    fn describe() {
        Object::describe()
    }
}

pub struct GPUPipelineLayoutDescriptor(Object);
impl GPUPipelineLayoutDescriptor {
    pub fn new(bind_group_layouts: Vec<GPUBindGroupLayout>) -> Result<Self, JsValue> {
        let o = Object::new();

        Reflect::set(
            &o,
            &"bindGroupLayouts".into(),
            &Array::from_iter(bind_group_layouts),
        )?;

        Ok(Self(o))
    }
}
impl From<GPUPipelineLayoutDescriptor> for Object {
    fn from(value: GPUPipelineLayoutDescriptor) -> Self {
        value.0
    }
}
impl IntoWasmAbi for GPUPipelineLayoutDescriptor {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPUPipelineLayoutDescriptor {
    fn describe() {
        Object::describe()
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GPUVertexAttribute {
    pub format: String,
    pub offset: usize,
    pub shader_location: u32,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GPUVertexBufferLayout {
    pub array_stride: usize,
    pub attributes: Vec<GPUVertexAttribute>,
}

pub struct GPURenderPipelineVertexProperties<'s> {
    pub entry_point: String,
    pub module: &'s GPUShaderModule,
    pub buffers: Option<Vec<GPUVertexBufferLayout>>,
}
impl GPURenderPipelineVertexProperties<'_> {
    pub fn into_object(self) -> Object {
        let o = Object::new();

        Reflect::set(&o, &"entryPoint".into(), &self.entry_point.into()).unwrap();
        Reflect::set(&o, &"module".into(), self.module).unwrap();
        if let Some(bs) = self.buffers {
            Reflect::set(
                &o,
                &"buffers".into(),
                &serde_wasm_bindgen::to_value(&bs).unwrap(),
            )
            .unwrap();
        }

        o
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GPURenderPipelineFragmentTarget {
    pub format: String,
}

pub struct GPURenderPipelineFragmentProperties<'s> {
    pub entry_point: String,
    pub module: &'s GPUShaderModule,
    pub targets: Vec<GPURenderPipelineFragmentTarget>,
}
impl GPURenderPipelineFragmentProperties<'_> {
    pub fn into_object(self) -> Object {
        let o = Object::new();

        Reflect::set(&o, &"entryPoint".into(), &self.entry_point.into()).unwrap();
        Reflect::set(&o, &"module".into(), self.module).unwrap();
        Reflect::set(
            &o,
            &"targets".into(),
            &serde_wasm_bindgen::to_value(&self.targets).unwrap(),
        )
        .unwrap();

        o
    }
}

pub struct GPURenderPipelineDescriptor(Object);
impl GPURenderPipelineDescriptor {
    pub fn new(layout: &GPUPipelineLayout, vertex: GPURenderPipelineVertexProperties) -> Self {
        let o = Object::new();

        Reflect::set(&o, &"layout".into(), layout).unwrap();
        Reflect::set(&o, &"vertex".into(), &vertex.into_object()).unwrap();

        Self(o)
    }

    pub fn fragment(self, fragment: GPURenderPipelineFragmentProperties) -> Self {
        Reflect::set(&self.0, &"fragment".into(), &fragment.into_object()).unwrap();

        self
    }
}
impl From<GPURenderPipelineDescriptor> for Object {
    fn from(value: GPURenderPipelineDescriptor) -> Self {
        value.0
    }
}
impl IntoWasmAbi for GPURenderPipelineDescriptor {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPURenderPipelineDescriptor {
    fn describe() {
        Object::describe()
    }
}

pub struct GPURenderBundleEncoderDescriptor(Object);
impl GPURenderBundleEncoderDescriptor {
    pub fn new(color_formats: Vec<String>) -> Self {
        let o = Object::new();

        Reflect::set(
            &o,
            &"colorFormats".into(),
            &color_formats
                .into_iter()
                .map(Into::<JsValue>::into)
                .collect::<Array>(),
        )
        .unwrap();

        Self(o)
    }
}
impl From<GPURenderBundleEncoderDescriptor> for Object {
    fn from(value: GPURenderBundleEncoderDescriptor) -> Self {
        value.0
    }
}
impl IntoWasmAbi for GPURenderBundleEncoderDescriptor {
    type Abi = <Object as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        Object::into_abi(self.0)
    }
}
impl wasm_bindgen::describe::WasmDescribe for GPURenderBundleEncoderDescriptor {
    fn describe() {
        Object::describe()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type HTMLCanvasElement;

    #[wasm_bindgen(method, js_name = getContext)]
    pub fn get_context(element: &HTMLCanvasElement, context_type: &str) -> JsValue;
}

#[wasm_bindgen]
extern "C" {

    pub type GPUCanvasContext;

    #[wasm_bindgen(method)]
    pub fn configure(ctx: &GPUCanvasContext, configuration: GPUCanvasConfiguration);
    #[wasm_bindgen(method, catch, js_name = getCurrentTexture)]
    pub fn get_current_texture(ctx: &GPUCanvasContext) -> Result<GPUTexture, JsValue>;

    pub type GPU;
    pub type GPUAdapter;
    pub type GPUSupportedFeatures;
    pub type GPUDevice;
    pub type GPUShaderModule;
    pub type GPUBindGroupLayout;
    pub type GPUPipelineLayout;
    pub type GPURenderPipeline;

    #[wasm_bindgen(typescript_type = "GPUBufferUsage")]
    type GPUBufferUsage;

    #[wasm_bindgen(js_namespace = navigator, js_name = gpu)]
    static NAVIGATOR_GPU: GPU;

    #[wasm_bindgen(method, js_name = requestAdapter)]
    async fn request_adapter(this: &GPU) -> JsValue;
    #[wasm_bindgen(method, js_name = getPreferredCanvasFormat)]
    pub fn get_preferred_canvas_format(gpu: &GPU) -> String;

    #[wasm_bindgen(method, getter)]
    fn features(this: &GPUAdapter) -> GPUSupportedFeatures;
    #[wasm_bindgen(method, js_name = requestDevice)]
    async fn request_device(this: &GPUAdapter) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn queue(this: &GPUDevice) -> GPUQueue;

    #[wasm_bindgen(method, js_name = createBuffer, catch)]
    fn create_buffer(
        this: &GPUDevice,
        descriptor: GPUBufferCreateDescription,
    ) -> Result<GPUBuffer, JsValue>;
    #[wasm_bindgen(method, js_name = createCommandEncoder, catch)]
    pub fn create_command_encoder(this: &GPUDevice) -> Result<GPUCommandEncoder, JsValue>;
    #[wasm_bindgen(method, js_name = createCommandEncoder, catch)]
    pub fn create_command_encoder_with_description(
        this: &GPUDevice,
        descriptor: GPUCommandEncoderCreateDescription,
    ) -> Result<GPUCommandEncoder, JsValue>;
    #[wasm_bindgen(method, js_name = createShaderModule, catch)]
    pub fn create_shader_module(
        device: &GPUDevice,
        descriptor: GPUShaderModuleDescriptor,
    ) -> Result<GPUShaderModule, JsValue>;
    #[wasm_bindgen(method, js_name = createPipelineLayout, catch)]
    pub fn create_pipeline_layout(
        device: &GPUDevice,
        descriptor: GPUPipelineLayoutDescriptor,
    ) -> Result<GPUPipelineLayout, JsValue>;
    #[wasm_bindgen(method, js_name = createRenderPipeline, catch)]
    pub fn create_render_pipeline(
        device: &GPUDevice,
        descriptor: GPURenderPipelineDescriptor,
    ) -> Result<GPURenderPipeline, JsValue>;
    #[wasm_bindgen(method, js_name = createRenderBundleEncoder, catch)]
    pub fn create_render_bundle_encoder(
        device: &GPUDevice,
        descriptor: GPURenderBundleEncoderDescriptor,
    ) -> Result<GPURenderBundleEncoder, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GPUBuffer;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &GPUBuffer);
    #[wasm_bindgen(method, js_name = mapAsync, catch)]
    pub async fn map_async(this: &GPUBuffer, mode: u32) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, js_name = mapAsync, catch)]
    pub async fn map_range_async(
        this: &GPUBuffer,
        mode: u32,
        offset: usize,
        size: usize,
    ) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method)]
    pub fn unmap(this: &GPUBuffer);
    #[wasm_bindgen(method, js_name = getMappedRange, catch)]
    pub fn get_mapped_range(
        this: &GPUBuffer,
        offset: usize,
        size: usize,
    ) -> Result<ArrayBuffer, JsValue>;
    #[wasm_bindgen(method, js_name = getMappedRange, catch)]
    pub fn get_mapped_range_from(this: &GPUBuffer, offset: usize) -> Result<ArrayBuffer, JsValue>;
    #[wasm_bindgen(method, js_name = getMappedRange, catch)]
    pub fn get_mapped_range_full(this: &GPUBuffer) -> Result<ArrayBuffer, JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn usage(this: &GPUBuffer) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn size(this: &GPUBuffer) -> usize;
}

#[wasm_bindgen]
extern "C" {
    pub type GPUTexture;
    pub type GPUTextureView;

    #[wasm_bindgen(method)]
    pub fn destroy(texture: &GPUTexture);
    #[wasm_bindgen(method, js_name = createView, catch)]
    pub fn create_view(texture: &GPUTexture) -> Result<GPUTextureView, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GPUCommandEncoder;
    pub type GPUCommandBuffer;

    #[wasm_bindgen(method, catch)]
    pub fn finish(this: &GPUCommandEncoder) -> Result<GPUCommandBuffer, JsValue>;
    #[wasm_bindgen(method, catch)]
    pub fn finish_with_descriptor(
        this: &GPUCommandEncoder,
        descriptor: GPUCommandBufferDescriptor,
    ) -> Result<GPUCommandBuffer, JsValue>;

    #[wasm_bindgen(method, js_name = copyBufferToBuffer, catch)]
    pub fn copy_buffer_to_buffer(
        this: &GPUCommandEncoder,
        source: &GPUBuffer,
        source_offset: usize,
        destination: &GPUBuffer,
        destination_offset: usize,
        size: usize,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = beginRenderPass, catch)]
    pub fn begin_render_pass(
        this: &GPUCommandEncoder,
        descriptor: GPURenderPassDescriptor,
    ) -> Result<GPURenderPassEncoder, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GPUQueue;

    #[wasm_bindgen(method, catch)]
    pub fn submit(queue: &GPUQueue, command_buffers: Vec<GPUCommandBuffer>) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GPURenderPassEncoder;

    #[wasm_bindgen(method, catch)]
    pub fn end(encoder: &GPURenderPassEncoder) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch, js_name = setPipeline)]
    pub fn set_pipeline(
        encoder: &GPURenderPassEncoder,
        pipeline: &GPURenderPipeline,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch, js_name = setScissorRect)]
    pub fn set_scissor_rect(
        encoder: &GPURenderPassEncoder,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch, js_name = setViewport)]
    pub fn set_viewport(
        encoder: &GPURenderPassEncoder,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch, js_name = setVertexBuffer)]
    pub fn set_vertex_buffer(
        encoder: &GPURenderPassEncoder,
        slot: u32,
        buffer: &GPUBuffer,
        offset: usize,
        size: usize,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn draw(
        encoder: &GPURenderPassEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
    #[wasm_bindgen(method, js_name = executeBundles, catch)]
    pub fn execute_bundles(
        encoder: &GPURenderPassEncoder,
        bundles: Vec<GPURenderBundle>,
    ) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type GPURenderBundleEncoder;
    pub type GPURenderBundle;

    #[wasm_bindgen(method, catch)]
    pub fn finish(encoder: &GPURenderBundleEncoder) -> Result<GPURenderBundle, JsValue>;
    #[wasm_bindgen(method, catch, js_name = setPipeline)]
    pub fn set_pipeline(
        encoder: &GPURenderBundleEncoder,
        pipeline: &GPURenderPipeline,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch, js_name = setVertexBuffer)]
    pub fn set_vertex_buffer(
        encoder: &GPURenderBundleEncoder,
        slot: u32,
        buffer: &GPUBuffer,
        offset: usize,
        size: usize,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(method)]
    pub fn draw(
        encoder: &GPURenderBundleEncoder,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    );
}

#[allow(non_snake_case)]
#[wasm_bindgen(js_namespace = GPUBufferUsage)]
extern "C" {
    #[wasm_bindgen]
    static COPY_SRC: u32;
    #[wasm_bindgen]
    static COPY_DST: u32;
    #[wasm_bindgen]
    static INDEX: u32;
    #[wasm_bindgen]
    static INDIRECT: u32;
    #[wasm_bindgen]
    static MAP_READ: u32;
    #[wasm_bindgen]
    static MAP_WRITE: u32;
    #[wasm_bindgen]
    static QUERY_RESOLVE: u32;
    #[wasm_bindgen]
    static STORAGE: u32;
    #[wasm_bindgen]
    static UNIFORM: u32;
    #[wasm_bindgen]
    static VERTEX: u32;
}

const SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f
}

@vertex
fn vsh(@location(0) pos: vec2f) -> VertexOutput {
    var vo: VertexOutput;

    vo.pos = vec4f(pos, 0.0, 1.0);
    vo.color = vec4f(pos, 1.0, 1.0);

    return vo;
}

@fragment
fn fsh(v: VertexOutput) -> @location(0) vec4f {
    return v.color;
}
"#;

#[wasm_bindgen]
pub async fn start(render_target_element: &HTMLCanvasElement) {
    #[cfg(feature = "panic_hook")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    log("hello from wasm");

    let gpu = &*NAVIGATOR_GPU;
    if gpu.is_null() {
        panic!("no webgpu available");
    }
    let adapter = gpu.request_adapter().await.unchecked_into::<GPUAdapter>();
    if adapter.is_null() {
        panic!("no adapters available");
    }

    log("webgpu is available on this platform");

    let features = adapter.features();
    let features_iter = js_sys::try_iter(&features)
        .expect("invalid")
        .expect("not iterable");
    for x in features_iter {
        log(&format!("feature: {x:?}"));
    }

    let ctx: GPUCanvasContext = render_target_element
        .get_context("webgpu")
        .try_into()
        .expect("not a canvas context");

    let device = adapter.request_device().await.unchecked_into::<GPUDevice>();
    let format = gpu.get_preferred_canvas_format();
    ctx.configure(GPUCanvasConfiguration::new(&device).with_format(&format));
    log(&format!("canvas was configured with format {format}"));

    let buffer = device
        .create_buffer(GPUBufferCreateDescription {
            label: None,
            mapped_at_creation: None,
            size: 128,
            usage: *VERTEX | *COPY_DST,
        })
        .expect("Failed to create buffer");
    let staging_buffer = device
        .create_buffer(GPUBufferCreateDescription {
            label: None,
            mapped_at_creation: Some(true),
            size: buffer.size() as _,
            usage: *COPY_SRC | *MAP_WRITE,
        })
        .expect("Failed to create staging buffer");
    let mapped_ab = staging_buffer.get_mapped_range_full().expect("not mapped?");
    Float32Array::new_with_byte_offset_and_length(&mapped_ab, 0, 6)
        .copy_from(&[0.0, -0.5, 0.75, 0.5, -0.75, 0.5]);
    staging_buffer.unmap();

    let copy_cmd = device
        .create_command_encoder()
        .expect("Failed to create command encoder");
    copy_cmd
        .copy_buffer_to_buffer(&staging_buffer, 0, &buffer, 0, 4 * 6)
        .expect("Failed to insert copy command");
    let copy_cmd = copy_cmd
        .finish()
        .expect("Failed to finish copy cmd recording");
    device
        .queue()
        .submit(vec![copy_cmd])
        .expect("Failed to send copy commands");

    let shader = device
        .create_shader_module(
            GPUShaderModuleDescriptor::new(SHADER)
                .expect("Failed to create shader module descriptor"),
        )
        .expect("Failed to create shader module");
    let empty_pl = device
        .create_pipeline_layout(
            GPUPipelineLayoutDescriptor::new(vec![])
                .expect("Failed to create pipeline layout descriptor"),
        )
        .expect("Failed to create pipeline layout");
    let render_pipeline = device
        .create_render_pipeline(
            GPURenderPipelineDescriptor::new(
                &empty_pl,
                GPURenderPipelineVertexProperties {
                    entry_point: "vsh".into(),
                    module: &shader,
                    buffers: Some(vec![GPUVertexBufferLayout {
                        array_stride: 4 * 2,
                        attributes: vec![GPUVertexAttribute {
                            format: "float32x2".into(),
                            offset: 0,
                            shader_location: 0,
                        }],
                    }]),
                },
            )
            .fragment(GPURenderPipelineFragmentProperties {
                entry_point: "fsh".into(),
                module: &shader,
                targets: vec![GPURenderPipelineFragmentTarget {
                    format: format.clone(),
                }],
            }),
        )
        .expect("Failed to create render pipeline");

    let triangle_render = device
        .create_render_bundle_encoder(GPURenderBundleEncoderDescriptor::new(vec![format]))
        .expect("Failed to create triangle render bundle encoder");
    triangle_render
        .set_pipeline(&render_pipeline)
        .expect("Failed to set render pipeline");
    triangle_render
        .set_vertex_buffer(0, &buffer, 0, 4 * 6)
        .expect("Failed to set vertex buffer");
    triangle_render.draw(3, 1, 0, 0);
    let triangle_render = triangle_render
        .finish()
        .expect("Failed to finish triangle render bundle");

    let render_target_view = ctx
        .get_current_texture()
        .expect("no current texture")
        .create_view()
        .expect("Failed to create target view");
    let main_target_attachment = GPURenderPassColorAttachment::new(&render_target_view)
        .clear_by_array(&[0.0, 0.0, 0.0, 1.0]);
    let render_pass = GPURenderPassDescriptor {
        color_attachments: vec![main_target_attachment],
    };

    let render_commands = device
        .create_command_encoder()
        .expect("Failed to begin render command recording");
    let rp = render_commands
        .begin_render_pass(render_pass)
        .expect("Failed to begin render pass");
    rp.execute_bundles(vec![triangle_render])
        .expect("Failed to execute render bundles");
    rp.end().expect("Failed to finish render pass");
    let render_commands = render_commands
        .finish()
        .expect("Failed to finish render commands");
    device
        .queue()
        .submit(vec![render_commands])
        .expect("Failed to submit render commands");
}
