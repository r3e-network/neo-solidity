import { existsSync, readFileSync, readdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig, type DefaultTheme } from 'vitepress';

const isCI = process.env.GITHUB_ACTIONS === 'true';
const docsRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');

type SidebarItem = DefaultTheme.SidebarItem;

function markdownPathForLink(link: string): string {
  const cleanLink = link.replace(/^\/+/, '').replace(/#.*$/, '');
  const pagePath = cleanLink.endsWith('/') ? `${cleanLink}index.md` : `${cleanLink}.md`;
  return resolve(docsRoot, pagePath);
}

function stripInlineMarkdown(text: string): string {
  return text
    .replace(/`([^`]+)`/g, '$1')
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
    .replace(/<[^>]+>/g, '')
    .replace(/\*\*([^*]+)\*\*/g, '$1')
    .trim();
}

function slugifyHeading(text: string): string {
  return stripInlineMarkdown(text)
    .normalize('NFKD')
    .replace(/[\u0300-\u036F]/g, '')
    .replace(/[\u0000-\u001f]/g, '')
    .replace(/[\s~`!@#$%^&*()\-_+=[\]{}|\\;:"'“”‘’<>,.?/]+/g, '-')
    .replace(/-{2,}/g, '-')
    .replace(/^-+|-+$/g, '')
    .replace(/^(\d)/, '_$1')
    .toLowerCase();
}

function hasHomeLayout(content: string): boolean {
  const frontmatter = content.match(/^---\n([\s\S]*?)\n---/);
  return Boolean(frontmatter?.[1].match(/^layout:\s*home\s*$/m));
}

function pageTitle(markdownPath: string): string {
  const content = readFileSync(markdownPath, 'utf8');
  const frontmatterTitle = content.match(/^title:\s*["']?(.+?)["']?\s*$/m);
  if (frontmatterTitle) {
    return frontmatterTitle[1];
  }

  return content.match(/^# (.+)$/m)?.[1] ?? markdownPath;
}

function pageSections(link: string): SidebarItem[] {
  const markdownPath = markdownPathForLink(link);
  if (!existsSync(markdownPath)) {
    return [];
  }

  const content = readFileSync(markdownPath, 'utf8');
  const sections: SidebarItem[] = [];

  for (const match of content.matchAll(/^(#{2,3}) (.+)$/gm)) {
    const heading = stripInlineMarkdown(match[2]);
    if (heading.length === 0) {
      continue;
    }

    const item = {
      text: heading,
      link: `${link}#${slugifyHeading(heading)}`
    };

    if (match[1] === '##' || sections.length === 0) {
      sections.push(item);
      continue;
    }

    const parent = sections[sections.length - 1];
    parent.items = [...(parent.items ?? []), item];
  }

  if (sections.length === 0 && /^#\s+.+$/m.test(content) && !hasHomeLayout(content)) {
    sections.push({
      text: 'Overview',
      link: `${link}#overview`
    });
  }

  return sections;
}

function page(text: string, link: string): SidebarItem {
  const sections = pageSections(link);
  return sections.length > 0 ? { text, link, items: sections } : { text, link };
}

function pageWithChildren(text: string, link: string, children: SidebarItem[]): SidebarItem {
  const items = [...pageSections(link), ...children];
  return items.length > 0 ? { text, link, items } : { text, link };
}

function standardsPages(category: string): SidebarItem[] {
  const categoryPath = resolve(docsRoot, `standards-mirror/${category}.md`);
  const categoryDir = resolve(docsRoot, `standards-mirror/${category}`);
  if (!existsSync(categoryPath) || !existsSync(categoryDir)) {
    return [];
  }

  const categoryContent = readFileSync(categoryPath, 'utf8');
  const ordered = [...categoryContent.matchAll(new RegExp(`/standards-mirror/${category}/([^\\)]+)`, 'g'))]
    .map((match) => match[1])
    .filter((slug, index, all) => all.indexOf(slug) === index);

  const slugs =
    ordered.length > 0
      ? ordered
      : readdirSync(categoryDir)
          .filter((file) => file.endsWith('.md'))
          .map((file) => file.replace(/\.md$/, ''))
          .sort();

  return slugs.map((slug) => {
    const link = `/standards-mirror/${category}/${slug}`;
    return page(pageTitle(markdownPathForLink(link)), link);
  });
}

function splitDocPages(base: string): SidebarItem[] {
  const indexPath = resolve(docsRoot, `${base}.md`);
  const sectionDir = resolve(docsRoot, base);
  if (!existsSync(indexPath) || !existsSync(sectionDir)) {
    return [];
  }

  const indexContent = readFileSync(indexPath, 'utf8');
  const ordered = [...indexContent.matchAll(new RegExp(`/${base}/([^\\)]+)`, 'g'))]
    .map((match) => match[1])
    .filter((slug, index, all) => all.indexOf(slug) === index);

  const slugs =
    ordered.length > 0
      ? ordered
      : readdirSync(sectionDir)
          .filter((file) => file.endsWith('.md'))
          .map((file) => file.replace(/\.md$/, ''))
          .sort();

  return slugs.map((slug) => {
    const link = `/${base}/${slug}`;
    return page(pageTitle(markdownPathForLink(link)), link);
  });
}

function originalContractPages(): SidebarItem[] {
  const indexPath = markdownPathForLink('/solidity/original-contracts/');
  if (!existsSync(indexPath)) {
    return [];
  }

  const groups: SidebarItem[] = [];
  let currentGroup: SidebarItem | undefined;

  for (const line of readFileSync(indexPath, 'utf8').split(/\r?\n/)) {
    const heading = line.match(/^## (.+)$/);
    if (heading) {
      currentGroup = heading[1] === 'Project Summary' ? undefined : { text: heading[1], collapsed: true, items: [] };
      if (currentGroup) {
        groups.push(currentGroup);
      }
      continue;
    }

    const row = line.match(/^\| \[([^\]]+)\]\((\/solidity\/original-contracts\/[^/)]+\/[^)]+)\)/);
    if (!row || !currentGroup) {
      continue;
    }

    currentGroup.items = [...(currentGroup.items ?? []), page(stripInlineMarkdown(row[1]), row[2])];
  }

  return groups.filter((group) => (group.items ?? []).length > 0);
}

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
    'plans/**',
    'ARCHITECTURE.md',
    'RUNTIME_SPEC.md',
    'ERROR_REFERENCE.md',
    'EXCELLENCE_ASSESSMENT.md',
    'FUZZ.md',
    'mapping_lowering_design.md',
    'NEO_VM_PARITY_TODO.md',
    'README.md',
    'SOLIDITY_SUPPORT_MATRIX.md',
    'public/**/*.md'
  ],
  markdown: {
    config(md) {
      md.core.ruler.after('block', 'neo_docs_overview_heading', (state) => {
        const tokens = state.tokens;

        if (
          hasHomeLayout(state.src) ||
          tokens.some((token) => token.type === 'heading_open' && /^h[2-3]$/.test(token.tag))
        ) {
          return;
        }

        const h1Index = tokens.findIndex((token) => token.type === 'heading_open' && token.tag === 'h1');
        if (h1Index === -1) {
          return;
        }

        const h1CloseIndex = tokens.findIndex(
          (token, index) => index > h1Index && token.type === 'heading_close' && token.tag === 'h1'
        );
        if (h1CloseIndex === -1) {
          return;
        }

        const openToken = new state.Token('heading_open', 'h2', 1);
        openToken.markup = '##';
        openToken.block = true;
        openToken.level = 0;
        const inlineToken = new state.Token('inline', '', 0);
        inlineToken.content = 'Overview';
        inlineToken.children = [];
        inlineToken.level = 1;
        const closeToken = new state.Token('heading_close', 'h2', -1);
        closeToken.markup = '##';
        closeToken.block = true;
        closeToken.level = 0;

        tokens.splice(h1CloseIndex + 1, 0, openToken, inlineToken, closeToken);
      });
    }
  },
  transformPageData(pageData) {
    if (
      pageData.headers.length === 0 &&
      pageData.frontmatter.layout !== 'home' &&
      pageData.relativePath.endsWith('.md')
    ) {
      pageData.headers.push({
        level: 2,
        title: 'Overview',
        slug: 'overview',
        link: '#overview',
        children: []
      });
    }
  },
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
      { text: 'Documentation', link: '/basics/introduction-to-smart-contracts' },
      { text: 'Mapping', link: '/mapping/' },
      { text: 'ERC ↔ Neo', link: '/standards-mirror/' },
      { text: 'Use cases', link: '/use-cases' },
      { text: 'Contribute', link: 'https://github.com/r3e-network/neo-solidity/blob/main/CONTRIBUTING.md' },
      { text: 'About', link: '/internals/architecture' },
      { text: 'Forum', link: 'https://github.com/r3e-network/neo-solidity/discussions' }
    ],
    sidebar: [
      {
        text: 'Basics',
        collapsed: false,
        items: [
          page('Introduction to Smart Contracts', '/basics/introduction-to-smart-contracts'),
          page('Solidity by Example', '/basics/solidity-by-example'),
          page('Installing the Compiler', '/basics/installing-the-compiler'),
          pageWithChildren('Quickstart', '/basics/quickstart', splitDocPages('basics/quickstart')),
          pageWithChildren('Deploying Contracts', '/basics/deploying-contracts', splitDocPages('basics/deploying-contracts')),
          pageWithChildren('Testing Contracts', '/basics/testing-contracts', splitDocPages('basics/testing-contracts')),
          page('Use Cases', '/use-cases')
        ]
      },
      {
        text: 'Language Description',
        collapsed: false,
        items: [
          pageWithChildren('EVM Feature Support', '/solidity/feature-support', splitDocPages('solidity/feature-support')),
          page('Layout of a Solidity Source File', '/language-description/layout-of-source-file'),
          page('Structure of a Contract', '/language-description/structure-of-a-contract'),
          page('Types', '/language-description/types'),
          page('Units and Globally Available Variables', '/language-description/units-and-global-variables'),
          page('Expressions and Control Structures', '/language-description/expressions-and-control-structures'),
          page('Contracts', '/language-description/contracts'),
          page('Inline Assembly', '/language-description/inline-assembly'),
          page('Cheatsheet', '/language-description/cheatsheet'),
          page('Language Grammar', '/language-description/grammar')
        ]
      },
      {
        text: 'Semantic Mapping',
        collapsed: false,
        items: [
          page('Overview', '/mapping/'),
          page('Execution Context', '/mapping/execution-context'),
          page('Types and Values', '/mapping/types-and-values'),
          page('Storage and Mappings', '/mapping/storage-and-mappings'),
          page('Calls and Assets', '/mapping/calls-and-assets'),
          page('Syscalls and Devpack', '/mapping/syscalls-and-devpack'),
          page('Parity and Limitations', '/mapping/parity-and-limitations'),
          page('Standards Mapping', '/mapping/standards'),
          page('Indexed Storage Lowering', '/mapping/indexed-storage-lowering')
        ]
      },
      {
        text: 'Compiler',
        collapsed: false,
        items: [
          page('Using the Compiler', '/compiler/using-the-compiler'),
          page('Analysing the Compiler Output', '/compiler/analysing-the-compiler-output'),
          page('IR-based Codegen Changes', '/compiler/ir-codegen-changes'),
          page('Fuzz Testing', '/compiler/fuzz-testing')
        ]
      },
      {
        text: 'Internals',
        collapsed: false,
        items: [
          page('Layout of State Variables in Storage and Transient Storage', '/internals/layout-in-storage'),
          page('Layout in Memory', '/internals/layout-in-memory'),
          page('Layout of Call Data', '/internals/layout-of-call-data'),
          page('Cleaning Up Variables', '/internals/cleaning-up-variables'),
          page('Source Mappings', '/internals/source-mappings'),
          page('The Optimizer', '/internals/the-optimizer'),
          pageWithChildren('Contract Metadata', '/internals/contract-metadata', splitDocPages('internals/contract-metadata')),
          page('Contract ABI Specification', '/internals/contract-abi-specification'),
          pageWithChildren('Architecture', '/internals/architecture', splitDocPages('internals/architecture')),
          pageWithChildren(
            'Runtime Specification',
            '/internals/runtime-specification',
            splitDocPages('internals/runtime-specification')
          ),
          pageWithChildren('Native Contracts', '/internals/native-contracts', splitDocPages('internals/native-contracts')),
          pageWithChildren('Syscalls', '/internals/syscalls', splitDocPages('internals/syscalls')),
          page('Parity & Limitations', '/internals/parity-and-limitations')
        ]
      },
      {
        text: 'Advisory Content',
        collapsed: false,
        items: [
          page('Security Considerations', '/advisory-content/security-considerations'),
          page('List of Known Bugs', '/advisory-content/known-bugs'),
          page('Breaking Changes', '/advisory-content/breaking-changes'),
          pageWithChildren(
            'Troubleshooting',
            '/advisory-content/troubleshooting',
            splitDocPages('advisory-content/troubleshooting')
          ),
          pageWithChildren(
            'Error Reference',
            '/advisory-content/error-reference',
            splitDocPages('advisory-content/error-reference')
          ),
          pageWithChildren(
            'Production Readiness',
            '/advisory-content/production-readiness',
            splitDocPages('advisory-content/production-readiness')
          )
        ]
      },
      {
        text: 'ERC / EIP ↔ Neo Mirror',
        collapsed: false,
        items: [
          page('Overview', '/standards-mirror/'),
          pageWithChildren('Token Standards', '/standards-mirror/tokens', standardsPages('tokens')),
          pageWithChildren(
            'Account & Authentication',
            '/standards-mirror/account-and-auth',
            standardsPages('account-and-auth')
          ),
          pageWithChildren(
            'Infrastructure & Patterns',
            '/standards-mirror/infrastructure',
            standardsPages('infrastructure')
          ),
          pageWithChildren('DeFi Building Blocks', '/standards-mirror/defi', standardsPages('defi')),
          pageWithChildren(
            'Protocol-Level EIPs',
            '/standards-mirror/protocol-eips',
            standardsPages('protocol-eips')
          ),
          pageWithChildren(
            'Deployment Reports',
            '/standards-mirror/deployments/README',
            [
              page('Last TestNet Results', '/standards-mirror/deployments/RESULTS'),
              page('Deferred Guardrails', '/standards-mirror/deployments/DEFERRED')
            ]
          )
        ]
      },
      {
        text: 'Additional Material',
        collapsed: false,
        items: [
          page('Famous Contracts Audit', '/solidity/famous-contracts-neo-audit'),
          page('Neo-Express Deployment Matrix', '/solidity/famous-contracts-neoxp-deploy'),
          page('Type-3 Runtime Execution', '/solidity/famous-contracts-neoxp-runtime'),
          page('TestNet Runtime Verification', '/solidity/famous-contracts-testnet-runtime'),
          pageWithChildren('Original Source Code', '/solidity/original-contracts/', originalContractPages()),
          page('NatSpec Format', '/additional-material/natspec-format'),
          page('SMTChecker and Formal Verification', '/additional-material/smtchecker'),
          page('Yul', '/additional-material/yul'),
          page('Import Path Resolution', '/additional-material/import-path-resolution'),
          pageWithChildren('Devpack Overview', '/additional-material/neo-devpack', splitDocPages('additional-material/neo-devpack')),
          pageWithChildren(
            'Standards and Contracts',
            '/additional-material/neo-standards',
            splitDocPages('additional-material/neo-standards')
          )
        ]
      },
      {
        text: 'Resources',
        collapsed: false,
        items: [
          page('Style Guide', '/resources/style-guide'),
          page('Common Patterns', '/resources/common-patterns'),
          page('Resources', '/resources/resources'),
          page('Contributing', '/resources/contributing'),
          page('Language Influences', '/resources/language-influences'),
          page('Solidity Brand Guide', '/resources/brand-guide'),
          page('Keyword Index', '/resources/keyword-index')
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
