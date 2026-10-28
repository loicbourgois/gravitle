const get_link = (links_view, link_size, idx) => {
	const link = {
		index: links_view.getUint32(idx * link_size + 0, true),
		a: links_view.getUint32(idx * link_size + 4, true),
		b: links_view.getUint32(idx * link_size + 8, true),
	};
	return link;
};
export { get_link };
