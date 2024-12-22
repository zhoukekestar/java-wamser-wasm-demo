package com.example;

public class WASMContainer {

    private static native String hello();

    private static native String handle(String request, WASMContainer context);

    static {
        System.loadLibrary("wasmlib");
    }

    public static void main(String[] args) {
        // example 1
        String helloMessage = WASMContainer.hello();
        System.out.println(helloMessage);

        String response = WASMContainer.handle("reqeust", new WASMContainer());
        System.out.println(" response =>  " + response);
    }

    public String invoke(String request) {
        return " " + request + " invoke_result ";
    }
}