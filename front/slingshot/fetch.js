const fetch_as_text = async (url) => {
	const response = await fetch(url);
	return await response.text();
};

const fetch_as_json_string = async (url) => {
	const response = await fetch(url);
	return JSON.stringify(await response.json());
};
export { fetch_as_text, fetch_as_json_string };
