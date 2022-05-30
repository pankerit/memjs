mod mem;

use neon::{prelude::*, types::buffer::TypedArray};
use windows::Win32::Foundation::*;

fn open_process(mut cx: FunctionContext) -> JsResult<JsObject> {
    let processName = cx.argument::<JsString>(0)?.value(&mut cx);
    let process = mem::open_process(processName.as_str()).unwrap();
    let mut obj = cx.empty_object();

    let id = cx.number(process.id);
    let name = cx.string(&process.name);
    let handle = cx.number(process.handle as f64);
    let path = cx.string(&process.path);

    obj.set(&mut cx, "id", id)?;
    obj.set(&mut cx, "name", name)?;
    obj.set(&mut cx, "handle", handle)?;
    obj.set(&mut cx, "path", path)?;

    Ok(obj)
}

fn close_handle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    mem::close_handle(HANDLE(handle as isize));
    Ok(cx.undefined())
}

fn sig_scan(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let pattern = cx.argument::<JsString>(1)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(2)?.value(&mut cx) as u64;

    let promise = cx
        .task(move || mem::sig_scan(HANDLE(handle as isize), &pattern, address))
        .promise::<JsValue, _>(move |mut cx, result| match result {
            Some(x) => Ok(cx.number(x as f64).upcast()),
            None => Ok(cx.undefined().upcast()),
        });
    Ok(promise)
}

fn read_memory(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;
    let size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let buffer: Vec<u8> = mem::read_memory_buffer(HANDLE(handle as isize), address, size);
    let a = JsBuffer::external(&mut cx, buffer);
    Ok(a)
}

fn write_memory(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;
    let data = cx.argument::<JsBuffer>(2)?.as_slice(&cx).to_vec();
    mem::write_memory_buffer(HANDLE(handle as isize), address, &data);
    Ok(cx.undefined())
}

fn alloc_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let address = mem::alloc_memory(HANDLE(handle as isize), size);
    Ok(cx.number(address as f64))
}

fn get_modules(mut cx: FunctionContext) -> JsResult<JsArray> {
    let process_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let modules = mem::get_modules(process_id as u32).unwrap();
    let obj = cx.empty_array();
    let mut i = 0;
    for module in modules {
        let module_obj = cx.empty_object();
        let id = cx.number(i as f64);
        let name = cx.string(&module.name);
        let base = cx.number(module.base_address as f64);
        let size = cx.number(module.size as f64);
        let path = cx.string(&module.path);

        module_obj.set(&mut cx, "id", id)?;
        module_obj.set(&mut cx, "name", name)?;
        module_obj.set(&mut cx, "base", base)?;
        module_obj.set(&mut cx, "size", size)?;
        module_obj.set(&mut cx, "path", path)?;

        obj.set(&mut cx, i, module_obj)?;
        i += 1;
    }
    Ok(obj)
}

fn get_processes(mut cx: FunctionContext) -> JsResult<JsObject> {
    let processes = mem::get_processes().unwrap();
    let obj = cx.empty_object();
    let mut i = 0;
    for process in processes {
        let process_obj = cx.empty_object();
        let id = cx.number(process.id as f64);
        let name = cx.string(&process.name);

        process_obj.set(&mut cx, "id", id)?;
        process_obj.set(&mut cx, "name", name)?;

        obj.set(&mut cx, i, process_obj)?;
        i += 1;
    }
    Ok(obj)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_process", open_process)?;
    cx.export_function("close_handle", close_handle)?;
    cx.export_function("sig_scan", sig_scan)?;
    cx.export_function("read_memory", read_memory)?;
    cx.export_function("write_memory", write_memory)?;
    cx.export_function("alloc_memory", alloc_memory)?;
    cx.export_function("get_modules", get_modules)?;
    cx.export_function("get_processes", get_processes)?;
    Ok(())
}
