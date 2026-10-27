import { defineConfig } from 'vitepress';

/** osbTK documentation site. */
export default defineConfig({
    title: 'osbTK',
    description: 'SDKs and tools for osu! storyboard development.',
    cleanUrls: true,
    themeConfig: {
        nav: [
            { text: 'Guide', link: '/getting-started' },
            { text: 'Reference', link: '/reference/cli' },
        ],
        sidebar: [
            {
                text: 'Guide',
                items: [
                    { text: 'Introduction', link: '/' },
                    { text: 'Getting started', link: '/getting-started' },
                    { text: 'How it works', link: '/how-it-works' },
                ],
            },
            {
                text: 'Reference',
                items: [
                    { text: 'CLI', link: '/reference/cli' },
                    { text: 'Error codes', link: '/reference/errors' },
                    { text: 'Supported C# subset', link: '/reference/csharp-subset' },
                ],
            },
        ],
        socialLinks: [{ icon: 'github', link: 'https://github.com/tochiResources/osb-toolkit' }],
    },
});
