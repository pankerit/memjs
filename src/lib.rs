pub mod mem;
use std::{cell::{RefCell}, sync::Arc};

use neon::{prelude::*, types::buffer::TypedArray};
    
fn open_process(mut cx: FunctionContext) -> JsResult<JsBox<Arc<mem::Process>>> {
    let processName = cx.argument::<JsString>(0)?.value(&mut cx);
    let process = mem::Process::new(processName.as_str()).unwrap();
    Ok(cx.boxed(Arc::new(process)))
}

fn close_handle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let process = cx.argument::<JsBox<Arc<mem::Process>>>(0)?;
    process.close_handle();
    Ok(cx.undefined())
}

fn sig_scan(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let process = cx.argument::<JsBox<Arc<mem::Process>>>(0)?.clone();
    let pattern = cx.argument::<JsString>(1)?.value(&mut cx);
    let address = cx.argument::<JsNumber>(2)?.value(&mut cx) as u64;

    let process = Arc::clone(&process);

    let promise = cx.task(move|| {
        process.sig_scan(&pattern, address)
    }).promise::<JsValue, _>(move|mut cx, result| {
        match result {
            Some(x) => {
                Ok(cx.number(x as f64).upcast())
            }
            None => {
                Ok(cx.undefined().upcast())
            }
        }
    });
    Ok(promise)
}

fn read_memory(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let process = cx.argument::<JsBox<RefCell<mem::Process>>>(0)?;
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;
    let size = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let process = process.borrow();
    let buffer = process.read_memory(address, size);
    let a = JsBuffer::external(&mut cx, buffer);
    Ok(a)
}

fn write_memory(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let process = cx.argument::<JsBox<RefCell<mem::Process>>>(0)?;
    let address = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;
    let data = cx.argument::<JsBuffer>(2)?.as_slice(&cx).to_vec();
    let process = process.borrow();
    process.write_memory(address, &data).unwrap();
    Ok(cx.undefined())
}



#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_process", open_process)?;
    cx.export_function("close_handle", close_handle)?;
    cx.export_function("sig_scan", sig_scan)?;
    cx.export_function("read_memory", read_memory)?;
    cx.export_function("write_memory", write_memory)?;
    Ok(())
}
