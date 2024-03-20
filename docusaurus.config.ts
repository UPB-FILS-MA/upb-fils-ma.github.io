import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

const config: Config = {
  title: 'Microprocessor Architecture',
  tagline: 'Use software to control hardware',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://embedded-rust-101.wyliodrin.com',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'UPB-FILS-MA', // Usually your GitHub org/user name.
  projectName: 'upb-fils-ma.github.io', // Usually your repo name.
  trailingSlash: false,

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/UPB-FILS-MA/upb-fils-ma.github.io/edit/main',
          remarkPlugins: [remarkMath],
          rehypePlugins: [rehypeKatex],
        },
        // blog: {
        //   showReadingTime: true,
        //   // Please change this to your repo.
        //   // Remove this to remove the "edit this page" links.
        //   editUrl:
        //     'https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/',
        // },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  stylesheets: [
    {
      href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
      type: 'text/css',
      integrity:
        'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
      crossorigin: 'anonymous',
    },
  ],

  markdown: {
    mermaid: true,
  },

  themes: ['@docusaurus/theme-mermaid'],

  themeConfig: {
    // Replace with your project's social card
    image: 'img/docusaurus-social-card.png',
    navbar: {
      title: 'Microprocessor Architecture',
      logo: {
        alt: 'Microprocessor Architecture',
        src: 'img/logo.svg',
      },
      items: [
        {to: '/docs/category/lecture', label: 'Lecture', position: 'left'},
        {to: '/docs/category/lab', label: 'Lab', position: 'left'},
        {to: '/docs/project', label: 'Project', position: 'left'},
        {to: '/docs/category/tutorials', label: 'Tutorials', position: 'left'},
        {
          href: 'https://github.com/UPB-FILS-MA/upb-fils-ma.github.io',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Materials',
          items: [
            {
              label: 'Lecture',
              to: '/docs/category/lecture',
            },
            {
              label: 'Lab',
              to: '/docs/category/lab',
            },
            {
              label: 'Project',
              to: '/docs/project',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            // {
            //   label: 'Stack Overflow',
            //   href: 'https://stackoverflow.com/questions/tagged/docusaurus',
            // },
            // {
            //   label: 'Discord',
            //   href: 'https://discordapp.com/invite/docusaurus',
            // },
            // {
            //   label: 'Twitter',
            //   href: 'https://twitter.com/docusaurus',
            // },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/upb-fils-ma',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} <a href="https://wyliodrin.com" target="_blank">Wyliodrin SRL</a>, licensed under CC BY-SA 4.0.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['linker-script', 'bash', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
