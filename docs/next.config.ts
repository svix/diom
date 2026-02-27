import type { NextConfig } from 'next'
import nextra from 'nextra'

// Set up Nextra with its configuration
const withNextra = nextra({
  // ... Add Nextra-specific options here
})

const nextConfig: NextConfig = {
  devIndicators: false,
  output: "export",
  images: {
    "unoptimized": true,
  },
}

export default withNextra(nextConfig)
