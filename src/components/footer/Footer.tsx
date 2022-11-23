const mail = "thomas.ung.pro@gmail.com";
const copyToClipboard = () => {
  navigator.clipboard.writeText(mail);
};

const Footer = () => {
  return (
    <footer className="footer bg-black text-white px-110px">
      <div className="container">
        <div id="contact-me" className="flex-col py-20">
          <h1>Contact me</h1>
          <div id="email flex-row">
            <a
              href="mailto:thomas.ung.pro@gmail.com"
              className="text-22px flex"
            >
              {mail}
            </a>

            <div className="flex hover:op80">
              <button
                className="i-carbon-copy-file text-22px"
                onClick={copyToClipboard}
              />
            </div>
          </div>
          <hr className="h-1px bg-white w-64" />
        </div>
        <div id="footer bottom row" className="flex-row">
          <div className="cr">
            Copyright 2022 © Designed by Cathy
          </div>
          <div id="socials">
            <ul className="list-none">
              <li className="flex justify-left hover:op80">
                <a
                  className="i-carbon-logo-github text-inherit"
                  href="https://github.com/tomkimsour"
                  target="_blank"
                />
              </li>
              <li className="flex justify-left hover:op80">
                <a
                  className="i-carbon-logo-linkedin text-inherit"
                  href="https://linkedin.com/in/thomas-ung"
                  target="_blank"
                />
              </li>
            </ul>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
