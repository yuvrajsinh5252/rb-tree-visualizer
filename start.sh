#!/bin/bash

echo "Installing dependencies..."
bun install

echo "Starting Tailwind CSS compiler..."
bunx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch &

echo "Starting Dioxus development server..."
dx serve --hot-reload