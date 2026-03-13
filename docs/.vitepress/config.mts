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
          { text: 'Introduction to Smart Contracts', link: '/basics/introduction-to-smart-contracts' },
          { text: 'Solidity by Example', link: '/basics/solidity-by-example' },
          { text: 'Installing the Compiler', link: '/basics/installing-the-compiler' }
        ]
      },
      {
        text: 'Language Description',
        collapsed: false,
        items: [
          { text: 'Layout of a Solidity Source File', link: '/language-description/layout-of-source-file' },
          { text: 'Structure of a Contract', link: '/language-description/structure-of-a-contract' },
          { text: 'Types', link: '/language-description/types' },
          { text: 'Units and Globally Available Variables', link: '/language-description/units-and-global-variables' },
          { text: 'Expressions and Control Structures', link: '/language-description/expressions-and-control-structures' },
          { text: 'Contracts', link: '/language-description/contracts' },
          { text: 'Inline Assembly', link: '/language-description/inline-assembly' },
          { text: 'Cheatsheet', link: '/language-description/cheatsheet' },
          { text: 'Language Grammar', link: '/language-description/grammar' }
        ]
      },
      {
        text: 'Compiler',
        collapsed: false,
        items: [
          { text: 'Using the Compiler', link: '/compiler/using-the-compiler' },
          { text: 'Analysing the Compiler Output', link: '/compiler/analysing-the-compiler-output' },
          { text: 'IR-based Codegen Changes', link: '/compiler/ir-codegen-changes' }
        ]
      },
      {
        text: 'Internals',
        collapsed: false,
        items: [
          { text: 'Layout in Storage', link: '/internals/layout-in-storage' },
          { text: 'Layout in Memory', link: '/internals/layout-in-memory' },
          { text: 'Layout of Call Data', link: '/internals/layout-of-call-data' },
          { text: 'Cleaning Up Variables', link: '/internals/cleaning-up-variables' },
          { text: 'Source Mappings', link: '/internals/source-mappings' },
          { text: 'The Optimizer', link: '/internals/the-optimizer' },
          { text: 'Contract Metadata', link: '/internals/contract-metadata' },
          { text: 'Contract ABI Specification', link: '/internals/contract-abi-specification' },
          { text: 'Architecture', link: '/internals/architecture' },
          { text: 'Runtime Specification', link: '/internals/runtime-specification' },
          { text: 'Parity & Limitations', link: '/internals/parity-and-limitations' }
        ]
      },
      {
        text: 'Advisory Content',
        collapsed: false,
        items: [
          { text: 'Security Considerations', link: '/advisory-content/security-considerations' },
          { text: 'List of Known Bugs', link: '/advisory-content/known-bugs' },
          { text: 'Breaking Changes', link: '/advisory-content/breaking-changes' },
          { text: 'Troubleshooting', link: '/advisory-content/troubleshooting' },
          { text: 'Error Reference', link: '/advisory-content/error-reference' }
        ]
      },
      {
        text: 'Additional Material',
        collapsed: false,
        items: [
          { text: 'NatSpec Format', link: '/additional-material/natspec-format' },
          { text: 'SMTChecker and Formal Verification', link: '/additional-material/smtchecker' },
          { text: 'Yul', link: '/additional-material/yul' },
          { text: 'Import Path Resolution', link: '/additional-material/import-path-resolution' },
          { text: 'Devpack Overview', link: '/additional-material/neo-devpack' },
          { text: 'Standards and Contracts', link: '/additional-material/neo-standards' }
        ]
      },
      {
        text: 'Resources',
        collapsed: false,
        items: [
          { text: 'Style Guide', link: '/resources/style-guide' },
          { text: 'Common Patterns', link: '/resources/common-patterns' },
          { text: 'Resources', link: '/resources/resources' },
          { text: 'Contributing', link: '/resources/contributing' },
          { text: 'Language Influences', link: '/resources/language-influences' },
          { text: 'Solidity Brand Guide', link: '/resources/brand-guide' }
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
