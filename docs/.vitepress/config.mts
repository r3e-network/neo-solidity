import { defineConfig } from 'vitepress';

const isCI = process.env.GITHUB_ACTIONS === 'true';

export default defineConfig({
  title: 'Neo Solidity',
  description:
    'Production-grade Solidity to Neo N3 compiler — compile .sol to .nef + .manifest.json with EVM-to-Neo semantic mapping',
  lang: 'en-US',
  base: isCI ? '/neo-solidity/' : '/',
  cleanUrls: true,
  lastUpdated: true,
  srcExclude: [
    'archive/**',
    'ARCHITECTURE.md',
    'RUNTIME_SPEC.md',
    'ERROR_REFERENCE.md',
    'EXCELLENCE_ASSESSMENT.md',
    'mapping_lowering_design.md',
    'NEO_VM_PARITY_TODO.md',
    'README.md',
    'SOLIDITY_SUPPORT_MATRIX.md',
    'public/**/*.md'
  ],
  head: [
    ['meta', { name: 'theme-color', content: '#00E599' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Neo Solidity — Compile Solidity to Neo N3' }],
    [
      'meta',
      {
        property: 'og:description',
        content:
          'Production-grade compiler that transforms Solidity 0.8.x contracts into deployable NeoVM artifacts (.nef + .manifest.json) with full EVM-to-Neo semantic mapping.'
      }
    ],
    ['meta', { property: 'og:image', content: '/assets/neo-solidity-logo.png' }],
    ['meta', { name: 'twitter:card', content: 'summary' }],
    ['meta', { name: 'twitter:title', content: 'Neo Solidity — Compile Solidity to Neo N3' }],
    [
      'meta',
      {
        name: 'twitter:description',
        content:
          'Production-grade Solidity to Neo N3 compiler with 113+ supported features, manifest hardening, and Neo-Express integration.'
      }
    ],
    ['link', { rel: 'icon', type: 'image/png', href: '/assets/neo-solidity-logo.png' }]
  ],
  themeConfig: {
    logo: '/assets/neo-solidity-logo.png',
    siteTitle: 'Neo Solidity',
    nav: [
      { text: 'Blog', link: 'https://medium.com/neo-smart-economy' },
      { text: 'Documentation', link: '/getting-started/overview' },
      { text: 'Use cases', link: '/use-cases' },
      { text: 'Contribute', link: 'https://github.com/r3e-network/neo-solidity/blob/main/CONTRIBUTING.md' },
      { text: 'About', link: '/reference/architecture' },
      { text: 'Forum', link: 'https://github.com/r3e-network/neo-solidity/discussions' }
    ],
    sidebar: [
      {
        text: 'Basics',
        collapsed: false,
        items: [
          { text: 'Introduction to Neo Solidity', link: '/getting-started/overview' },
          { text: 'Installing the Compiler', link: '/getting-started/installation' },
          { text: 'Quick Start', link: '/getting-started/quickstart' },
          { text: 'Neo Solidity by Example', link: '/getting-started/solidity-by-example' }
        ]
      },
      {
        text: 'Language Description',
        collapsed: false,
        items: [
          { text: 'EVM to NeoVM Mapping', link: '/mapping/evm-to-neovm' },
          { text: 'Feature Support Matrix', link: '/solidity/feature-support' },
          { text: 'Syntax and Behavior', link: '/solidity/syntax-and-behavior' }
        ]
      },
      {
        text: 'Compiler',
        collapsed: false,
        items: [
          { text: 'Using the CLI', link: '/reference/cli' },
          { text: 'Compile Workflows', link: '/workflows/compile' },
          { text: 'Manifest & Permissions', link: '/manifests/manifest-spec' },
          { text: 'Devpack & Standards', link: '/devpack/overview' },
          { text: 'Standard Contracts', link: '/devpack/standards' }
        ]
      },
      {
        text: 'Internals',
        collapsed: false,
        items: [
          { text: 'Compiler Architecture', link: '/reference/architecture' },
          { text: 'NeoVM Runtime Spec', link: '/reference/runtime' },
          { text: 'Native Contracts', link: '/neovm/native-contracts' },
          { text: 'Syscalls', link: '/neovm/syscalls' },
          { text: 'Parity & Limitations', link: '/reference/parity-limitations' }
        ]
      },
      {
        text: 'Advisory Content',
        collapsed: false,
        items: [
          { text: 'Testing Contracts', link: '/workflows/test' },
          { text: 'Deploying Contracts', link: '/workflows/deploy' },
          { text: 'Production Readiness', link: '/workflows/production' },
          { text: 'Troubleshooting', link: '/reference/troubleshooting' },
          { text: 'Error Codes', link: '/reference/errors' }
        ]
      },
      {
        text: 'Additional Material',
        collapsed: false,
        items: [
          { text: 'Famous Contracts Audit', link: '/solidity/famous-contracts-neo-audit' },
          { text: 'Type-3 Runtime Tests', link: '/solidity/famous-contracts-neoxp-runtime' },
          { text: 'Original Source Code', link: '/solidity/original-contracts/' }
        ]
      }
    ],
    socialLinks: [{ icon: 'github', link: 'https://github.com/r3e-network/neo-solidity' }],
    footer: {
      message: 'MIT Licensed',
      copyright: 'Copyright © R3E Network'
    },
    search: {
      provider: 'local'
    },
    outline: {
      level: [2, 3]
    },
    editLink: {
      pattern: 'https://github.com/r3e-network/neo-solidity/edit/main/docs/:path',
      text: 'Edit this page on GitHub'
    }
  }
});
