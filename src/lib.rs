use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub const GPU_BUFFER_USAGE_MAP_READ: u32 = 0x01;
pub const GPU_BUFFER_USAGE_MAP_WRITE: u32 = 0x02;
pub const GPU_BUFFER_USAGE_COPY_SRC: u32 = 0x04;
pub const GPU_BUFFER_USAGE_COPY_DST: u32 = 0x08;
pub const GPU_BUFFER_USAGE_VERTEX: u32 = 0x20;
pub const GPU_BUFFER_USAGE_UNIFORM: u32 = 0x40;

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
extern "C" {
    type GPU;
    type GPUAdapter;
    type GPUSupportedFeatures;
    type GPUDevice;
    type GPUBuffer;

    #[wasm_bindgen(js_namespace = navigator, js_name = gpu)]
    static NAVIGATOR_GPU: GPU;

    #[wasm_bindgen(method, js_name = requestAdapter)]
    async fn request_adapter(this: &GPU) -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn features(this: &GPUAdapter) -> GPUSupportedFeatures;
    #[wasm_bindgen(method, js_name = requestDevice)]
    async fn request_device(this: &GPUAdapter) -> JsValue;

    #[wasm_bindgen(method, js_name = createBuffer)]
    fn create_buffer(this: &GPUDevice, descriptor: GPUBufferCreateDescription) -> GPUBuffer;
}

#[wasm_bindgen]
pub async fn start() {
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
        log(&format!("{x:?}"));
    }

    let device = adapter.request_device().await.unchecked_into::<GPUDevice>();

    let buffer = device.create_buffer(GPUBufferCreateDescription {
        label: None,
        mapped_at_creation: Some(false),
        size: 128,
        usage: GPU_BUFFER_USAGE_VERTEX | GPU_BUFFER_USAGE_COPY_DST,
    });
    if buffer.is_null() {
        panic!("Failed to create buffer");
    }
}
