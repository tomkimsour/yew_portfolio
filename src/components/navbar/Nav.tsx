import data from "../../assets/nav.json" assert { type: "json" };
import NavItem from "./navComponent/NavItem.tsx";

const Nav = () => {
  return (
    <nav className="fixed w-full mt-0 pr-40 py-14 justify-end">
      <ul className="list-none">
        {data.navItems.map((navItem, key) => (
          <NavItem key={key} name={navItem.name} to={navItem.to} />
        ))}
      </ul>
    </nav>
  );
};

export default Nav;
