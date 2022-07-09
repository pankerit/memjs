mod mem;

use neon::{prelude::*, types::buffer::TypedArray};
use windows::Win32::Foundation::*;

fn open_process(mut cx: FunctionContext) -> JsResult<JsObject> {
    let process_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let process = match mem::open_process(process_name.as_str()) {
        Ok(process) => process,
        Err(err) => {
            return cx.throw_error(err.to_string());
        }
    };

    let obj = cx.empty_object();
    let id = cx.number(process.id);
    let name = cx.string(&process.name);
    let handle = cx.number(process.handle.0 as f64);

    obj.set(&mut cx, "id", id)?;
    obj.set(&mut cx, "name", name)?;
    obj.set(&mut cx, "handle", handle)?;

    Ok(obj)
}

fn close_handle(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let result = mem::close_handle(HANDLE(handle as isize));
    Ok(cx.boolean(result))
}

fn sig_scan_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let signature = cx.argument::<JsString>(1)?.value(&mut cx);
    let base_address = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
    let result = mem::sig_scan(HANDLE(handle as isize), signature.as_str(), base_address);
    match result {
        Some(x) => Ok(cx.number(x as f64).upcast()),
        None => Ok(cx.undefined().upcast()),
    }
}

fn sig_scan(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let signature = cx.argument::<JsString>(1)?.value(&mut cx);
    let base_address = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;

    let promise = cx
        .task(move || mem::sig_scan(HANDLE(handle as isize), &signature, base_address))
        .promise::<JsValue, _>(move |mut cx, result| match result {
            Some(x) => Ok(cx.number(x as f64).upcast()),
            None => Ok(cx.undefined().upcast()),
        });
    Ok(promise)
}

fn sig_scan_module_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let process_id = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let signature = cx.argument::<JsString>(2)?.value(&mut cx);
    let module_name = cx.argument::<JsString>(3)?.value(&mut cx);
    let result = mem::sig_scan_module(
        HANDLE(handle as isize),
        process_id,
        &signature,
        &module_name,
    );
    match result {
        Some(x) => Ok(cx.number(x as f64).upcast()),
        None => Ok(cx.undefined().upcast()),
    }
}

fn sig_scan_module(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let process_id = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let signature = cx.argument::<JsString>(2)?.value(&mut cx);
    let module_name = cx.argument::<JsString>(3)?.value(&mut cx);

    let promise = cx
        .task(move || {
            mem::sig_scan_module(
                HANDLE(handle as isize),
                process_id,
                &signature,
                &module_name,
            )
        })
        .promise::<JsValue, _>(move |mut cx, result| match result {
            Some(x) => Ok(cx.number(x as f64).upcast()),
            None => Ok(cx.undefined().upcast()),
        });
    Ok(promise)
}

fn read_memory_buffer(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let buffer: Vec<u8> = mem::read_memory_buffer(HANDLE(handle as isize), address, size);
    let a = JsBuffer::external(&mut cx, buffer);
    Ok(a)
}

fn write_memory_buffer(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let buffer = cx.argument::<JsBuffer>(2)?.as_slice(&cx).to_vec();
    mem::write_memory_buffer(HANDLE(handle as isize), address, &buffer);
    Ok(cx.undefined())
}

fn alloc_memory(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let size = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let address = mem::alloc_memory(HANDLE(handle as isize), size);
    Ok(cx.number(address as f64))
}

fn read_memory_u32(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<u32>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_u32(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_u64(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<u64>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_u64(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as u64;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_i32(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<i32>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_i32(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as i32;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_i64(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<i64>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_i64(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as i64;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_f32(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<f32>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_f32(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as f32;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_f64(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<f64>(HANDLE(handle as isize), address);
    Ok(cx.number(value as f64))
}

fn write_memory_f64(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsNumber>(2)?.value(&mut cx) as f64;
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_bool(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = mem::read_memory::<bool>(HANDLE(handle as isize), address);
    Ok(cx.boolean(value))
}

fn write_memory_bool(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let value = cx.argument::<JsBoolean>(2)?.value(&mut cx);
    mem::write_memory(HANDLE(handle as isize), address, value);
    Ok(cx.undefined())
}

fn read_memory_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let buffer = mem::read_memory_buffer(HANDLE(handle as isize), address, size);
    let value = String::from_utf8(buffer).unwrap();
    Ok(cx.string(&value))
}

fn write_memory_string(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let string = cx.argument::<JsString>(2)?.value(&mut cx);
    let buffer = string.into_bytes();
    mem::write_memory_buffer(HANDLE(handle as isize), address, &buffer);
    Ok(cx.undefined())
}

fn get_process_path(mut cx: FunctionContext) -> JsResult<JsValue> {
    let handle = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let path = mem::get_process_path(HANDLE(handle as isize));
    match path {
        Some(path) => Ok(cx.string(&path).upcast()),
        None => Ok(cx.undefined().upcast()),
    }
}

fn get_process_modules(mut cx: FunctionContext) -> JsResult<JsArray> {
    let process_id = cx.argument::<JsNumber>(0)?.value(&mut cx) as u32;
    let modules = mem::get_process_modules(process_id);
    let array = cx.empty_array();
    let mut i = 0;
    for module in modules {
        let object = cx.empty_object();
        let name = cx.string(&module.name);
        let base_address = cx.number(module.base_address as f64);
        let size = cx.number(module.size as f64);
        let path = cx.string(&module.path);

        object.set(&mut cx, "path", path)?;
        object.set(&mut cx, "name", name)?;
        object.set(&mut cx, "baseAddress", base_address)?;
        object.set(&mut cx, "size", size)?;

        array.set(&mut cx, i, object)?;
        i += 1;
    }
    Ok(array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_process", open_process)?;
    cx.export_function("close_handle", close_handle)?;
    cx.export_function("sig_scan_sync", sig_scan_sync)?;
    cx.export_function("sig_scan", sig_scan)?;
    cx.export_function("sig_scan_module_sync", sig_scan_module_sync)?;
    cx.export_function("sig_scan_module", sig_scan_module)?;
    cx.export_function("read_memory_buffer", read_memory_buffer)?;
    cx.export_function("write_memory_buffer", write_memory_buffer)?;
    cx.export_function("alloc_memory", alloc_memory)?;
    cx.export_function("read_memory_u32", read_memory_u32)?;
    cx.export_function("write_memory_u32", write_memory_u32)?;
    cx.export_function("read_memory_u64", read_memory_u64)?;
    cx.export_function("write_memory_u64", write_memory_u64)?;
    cx.export_function("read_memory_i32", read_memory_i32)?;
    cx.export_function("write_memory_i32", write_memory_i32)?;
    cx.export_function("read_memory_i64", read_memory_i64)?;
    cx.export_function("write_memory_i64", write_memory_i64)?;
    cx.export_function("read_memory_f32", read_memory_f32)?;
    cx.export_function("write_memory_f32", write_memory_f32)?;
    cx.export_function("read_memory_f64", read_memory_f64)?;
    cx.export_function("write_memory_f64", write_memory_f64)?;
    cx.export_function("read_memory_bool", read_memory_bool)?;
    cx.export_function("write_memory_bool", write_memory_bool)?;
    cx.export_function("read_memory_string", read_memory_string)?;
    cx.export_function("write_memory_string", write_memory_string)?;
    cx.export_function("get_process_path", get_process_path)?;
    cx.export_function("get_process_modules", get_process_modules)?;
    Ok(())
}
