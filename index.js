async function main() {
  const { hello } = await import('./pkg');

  alert(hello());
};

main();
