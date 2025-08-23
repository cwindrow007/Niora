module.exports = {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                'maya-blue': '#55CDFC',
                'amouranth-pink': '#F7A8B8',
                'pink-200': '#FBCFE8',
            },
            backgroundSize: {
                '300': '300% 300%',
                '400': '400% 100%',
            },
            animation: {
                'gradient-x': 'gradient-x 6s ease-in-out infinite',
            },
            keyframes: {
                'gradient-x': {
                    '0%, 100%': {'background-position': '0% 50%',},
                    '50%': {'background-position': '100% 50%',},
                },
            },
        },
    },
    plugins: [],
}