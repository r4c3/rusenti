import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
    try {
        const response = await fetch('http://127.0.0.1:3000/');
        const data = await response.text();
        return { data };
    } catch (error) {
        console.error('Failed to fetch data:', error);
        return { data: 'Error fetching data' };
    }
};
