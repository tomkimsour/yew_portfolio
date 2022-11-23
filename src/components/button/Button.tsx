const Button = (props: { name: string }) => {
  return (
    <div>
      <button className="border-1 border-black text-black font-bold 
      hover:bg-black hover:text-white  py-2 px-8 rounded-full">
        {props.name}
      </button>
    </div>
  );
};

export default Button;
