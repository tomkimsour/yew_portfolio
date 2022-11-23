import { LanguageCategory } from "./Interfaces.ts";

const Languages = (props: LanguageCategory) => {
  return (
    <div>
      <h2 className="categoryTitle">{props.categoryTitle}</h2>
      {props.languages.map((value, key: number) => {
        return (
          <div className="language" key={key}>
            <div className="title">
              {value.name}
            </div>
            <div className="level">{value.level}</div>
          </div>
        );
      })}
    </div>
  );
};

export default Languages;
