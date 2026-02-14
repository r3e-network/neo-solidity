import { defineConfig } from 'vitepress';

const isCI = process.env.GITHUB_ACTIONS === 'true';

export default defineConfig({
  title: 'Neo Solidity',
  description: 'Production-grade Solidity to Neo N3 compiler and devpack documentation',
  lang: 'en-US',
  base: isCI ? '/neo-solidity/' : '/',
  cleanUrls: true,
  lastUpdated: true,
  srcExclude: ['archive/**'],
  head: [
    ['meta', { name: 'theme-color', content: '#0f4c81' }],
    ['meta', { property: 'og:title', content: 'Neo Solidity Documentation' }],
    [
      'meta',
      {
        property: 'og:description',
        content:
          'Compile Solidity to Neo N3 (.nef + .manifest.json), with complete deployment and runtime guidance.'
      }
    ]
  ],
  themeConfig: {
    logo: '/assets/neo-solidity-logo.svg',
    siteTitle: 'Neo Solidity',
    nav: [
      { text: 'Getting Started', link: '/getting-started/overview' },
      { text: 'Workflows', link: '/workflows/compile' },
      { text: 'Solidity', link: '/solidity/feature-support' },
      { text: 'Mapping', link: '/mapping/evm-to-neovm' },
      { text: 'NeoVM', link: '/neovm/native-contracts' },
      { text: 'Manifest', link: '/manifests/manifest-spec' },
      { text: 'Devpack', link: '/devpack/overview' },
      { text: 'Reference', link: '/reference/cli' }
    ],
    sidebar: {
      '/getting-started/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Overview', link: '/getting-started/overview' },
            { text: 'Installation', link: '/getting-started/installation' },
            { text: 'Quick Start', link: '/getting-started/quickstart' }
          ]
        }
      ],
      '/workflows/': [
        {
          text: 'Workflows',
          items: [
            { text: 'Compile Contracts', link: '/workflows/compile' },
            { text: 'Deploy Contracts', link: '/workflows/deploy' },
            { text: 'Test Contracts', link: '/workflows/test' },
            { text: 'Production Readiness', link: '/workflows/production' }
          ]
        }
      ],
      '/solidity/': [
        {
          text: 'Solidity on Neo',
          items: [
            { text: 'Feature Support', link: '/solidity/feature-support' },
            { text: 'Syntax and Behavior', link: '/solidity/syntax-and-behavior' }
          ]
        }
      ],
      '/mapping/': [
        {
          text: 'EVM to NeoVM Mapping',
          items: [{ text: 'Semantic Mapping', link: '/mapping/evm-to-neovm' }]
        }
      ],
      '/neovm/': [
        {
          text: 'NeoVM Runtime',
          items: [
            { text: 'Native Contracts', link: '/neovm/native-contracts' },
            { text: 'Syscalls', link: '/neovm/syscalls' }
          ]
        }
      ],
      '/manifests/': [
        {
          text: 'Manifest System',
          items: [{ text: 'Manifest Spec and Policy', link: '/manifests/manifest-spec' }]
        }
      ],
      '/devpack/': [
        {
          text: 'Devpack',
          items: [
            { text: 'Overview', link: '/devpack/overview' },
            { text: 'Standards and Contracts', link: '/devpack/standards' }
          ]
        }
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'CLI Reference', link: '/reference/cli' },
            { text: 'Manifest Reference', link: '/manifests/manifest-spec' },
            { text: 'Error Reference', link: '/reference/errors' },
            { text: 'Runtime Specification', link: '/reference/runtime' },
            { text: 'Architecture', link: '/reference/architecture' },
            { text: 'Parity and Limitations', link: '/reference/parity-limitations' }
          ]
        }
      ]
    },
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
