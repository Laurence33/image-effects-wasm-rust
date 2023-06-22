async function init() {
  let rustApp = null;
  try {
    rustApp = await import('../pkg');
  } catch (e) {
    console.error(e);
    return;
  }
  const input = document.getElementById('upload');

  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ''
    );
    console.log(base64);
    console.log(input.files[0]);
  };

  input.addEventListener('change', (e) => {
    fileReader.readAsDataURL(e.target.files[0]);
  });
}

init();
