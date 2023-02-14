{
  description = "Panic's Assorted Templates";

  inputs = {
    # nixpkgs.url = "nixpkgs/nixos-21.11";
    # nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    # utils = { url = "github:numtide/flake-utils"; };
    # rust-overlay = { url = "github:oxalica/rust-overlay"; };
    # flake-compat = {
    #   url = "github:edolstra/flake-compat"; 
    #   flake = false;
    # };
  };

  outputs = { self, ... }@inputs: {
    templates = {
      
      golang-basic = {
        path = ./golang-basic;
        description = "Golang project.";
      };
      python-basic = {
        path = ./python-basic;
        description = "Python project.";
      };
      rust-aws-sam = {
        path = ./rust-aws-sam;
        description = "Rust in AWS Lambda with SAM.";
      };
      rust-basic = {
        path = ./rust-basic;
        description = "Rust project.";
      };
      rust-dioxus = {
        path = ./rust-dioxus;
        description = "Dioxus cross-platform GUI app.";
      };
      rust-iot-nrf52 = {
        path = ./rust-iot-nrf52;
        description = "Rust IoT on nRF52.";
      };
      rust-iot-stm32 = {
        path = ./rust-iot-stm32;
        description = "Rust IoT on STM32.";
      };
      rust-node = {
        path = ./rust-node;
        description = "Rust and NodeJS hybrid project.";
      };
      rust-yew = {
        path = ./rust-yew;
        description = "Yew Web app.";
      };
      tex-basic = {
        path = ./tex-basic;
        description = "LaTeX Document.";
      };
      vagrant-basic = {
        path = ./vagrant-basic;
        description = "Vagrant boilerplate.";
      };
    
    };

  };

}
