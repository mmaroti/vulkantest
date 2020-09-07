use std::sync::Arc;
use vulkano::buffer::*;
use vulkano::command_buffer::*;
use vulkano::descriptor::descriptor_set::*;
use vulkano::descriptor::*;
use vulkano::device::*;
use vulkano::instance::*;
use vulkano::pipeline::*;
use vulkano::sync::*;

mod test {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "src/test.glsl",
    }
}

fn main() {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("vulkan is not available");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!("id: {}, queues: {}, graphics: {}, compute: {}, transfers: {}", family.id(), family.queues_count(), family.supports_graphics(), family.supports_compute(), family.explicitly_supports_transfers());
    }

    let family = physical
        .queue_families()
        .find(|&q| q.supports_compute())
        .expect("no compute queue family");

    let extension = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ..DeviceExtensions::none()
    };

    let (device, mut queues) = Device::new(
        physical,
        &Features::none(),
        &extension,
        [(family, 0.5)].iter().cloned(),
    )
    .unwrap();

    let queue = queues.next().unwrap();
    // println!("{:?}", device.enabled_features());

    let usage = BufferUsage {
        storage_buffer: true,
        transfer_destination: true,
        ..BufferUsage::none()
    };

    let data = 0..20;

    let buffer = CpuAccessibleBuffer::from_iter(device.clone(), usage, false, data).unwrap();

    println!("{:?}", buffer.read().unwrap().as_ref());

    let shader = test::Shader::load(device.clone()).unwrap();
    // println!("{:?}", shader.module());

    let specialization = test::SpecializationConstants { OFFSET: 200 };

    let pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &specialization).unwrap(),
    );

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();

    let descriptor = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );
    // let descriptor = PersistentDescriptorSet::start(test::Layout::default());

    let mut builder = AutoCommandBufferBuilder::primary(device.clone(), queue.family()).unwrap();

    builder
        .dispatch([1024, 1, 1], pipeline, descriptor, ())
        .unwrap();

    let command = Arc::new(builder.build().unwrap());

    let finished = command
        .clone()
        .execute(queue.clone())
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    finished.wait(None).unwrap();

    println!("{:?}", buffer.read().unwrap().as_ref());
}
