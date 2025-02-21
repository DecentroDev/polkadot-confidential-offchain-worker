import { useRouter } from 'next/router';
import type { DocsThemeConfig } from 'nextra-theme-docs';
import React from 'react';
import packageJson from './package.json' assert { type: 'json' };

const config: DocsThemeConfig = {
  logo: <span>Polkadot Confidential Offchain Worker</span>,
  project: {
    link: 'https://github.com/DecentroDev/polkadot-confidential-offchain-worker.git',
  },
  docsRepositoryBase: 'https://github.com/DecentroDev/polkadot-confidential-offchain-worker.git/tree/main/docs',
  feedback: {
    content: null,
  },
  footer: {
    text: <span className='text-sm'>{new Date().getFullYear()} © PRMX all rights reserved</span>,
  },
  sidebar: {
    toggleButton: true,
    defaultMenuCollapseLevel: 1,
  },
  search: {
    placeholder: 'Search website...',
  },
  // banner: {
  //   dismissible: true,
  //   key: `version-${packageJson.version}`,
  //   text: (
  //     <a href='http://' target='_blank' rel='noopener noreferrer'>
  //       Polkadot Sufficient asset is in public beta →
  //     </a>
  //   ),
  // },
  toc: {
    float: true,
    backToTop: true,
  },
  nextThemes: {
    defaultTheme: 'dark',
  },
  useNextSeoProps() {
    const { asPath, pathname, query } = useRouter();

    if (['/', '/docs'].includes(asPath)) {
      return { titleTemplate: 'Confidential API Off-Chain Worker' };
    }

    if (pathname.startsWith('/wrapped/')) {
      return {
        title: (query?.name ?? 'React app') + ' Wrapped | Confidential API Off-Chain Worker',
      };
    }

    return { titleTemplate: `%s | Confidential API Off-Chain Worker` };
  },
  primarySaturation: 0,
};

export default config;
