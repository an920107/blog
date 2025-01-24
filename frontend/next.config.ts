import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Avoid from rendering twice in development mode
  reactStrictMode: false,

  output: "standalone",
  env: {
    APP_VERSION: process.env.npm_package_version,
  },
};

export default nextConfig;
