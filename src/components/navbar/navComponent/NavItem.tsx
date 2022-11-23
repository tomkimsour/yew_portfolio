import { Link } from "aleph/react";

const NavItem = (props: { to: string; name: string }) => {
  return (
    <li className="inline-block">
      <Link to={props.to} className="px-15px">
        {props.name}
      </Link>
    </li>
  );
};

export default NavItem;
