import Button from "../Button/Button.tsx";

const Home = () => {
  return (
    <div className=" flex flex-nowrap box-border px-110px min-h-screen">
      <div className="min-w-1/2 bg-red-700	">
        <img
          className="object-cover w-100 h-85 pl-50"
          src="../assets/thomas-2022.jpeg"
          alt="picture of thomas ung"
        />
      </div>
      <div className="pt-63 ml-76 mr-24 w-1/2 h-full">
        <div className="pb-3">
          <h1 className="font-bold">
            Thomas UNG
          </h1>
          <h2 className="pb-3">Research Engineer</h2>
          <p>
            Bonjour, bonjour ! I'm a french research engineer that works in the
            robotic field. On the side, I like learning various stuff and doing
            some sport. Feel free to give a look around the website for more
            information
          </p>
        </div>
        <div>
          <Button name={"ABOUT"} />
        </div>
      </div>
    </div>
  );
};
export default Home;
