async function init() {
  let rustApp = null;

  try {
    rustApp = await import('../pkg');
  } catch (error) {
    console.error(`Unexpected error in load rust app: ${error}`);
    return;
  }

  console.log('Rust app loaded', rustApp);

  const inputUpload = document.getElementById('upload');
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(/^data:image\/(png|jpg|jpeg);base64,/, '');

    let img_data_url = rustApp.gray_scale(base64);

    document.getElementById('new-img').setAttribute('src', img_data_url);
  };

  inputUpload.addEventListener('change', () => {
    fileReader.readAsDataURL(inputUpload.files[0]);
  });
}

init();
