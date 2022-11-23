import { Formation, FormationCategory } from "./Interfaces.ts";
import {
  FormationDescription,
  FormationName,
  FormationPlace,
  FormationYear,
} from "./FormationComponent/index.ts";

const Formation = (props: FormationCategory) => {
  return (
    <section className="education">
      <h2>{props.categoryTitle}</h2>
      {props.formations.map((value, key: number) => {
        return (
          <div className="formation" key={key}>
            <div className="left-wrapper">
              <FormationName name={value.name} />
              <FormationDescription
                description={value.description}
              />
            </div>
            <div className="right-wrapper">
              <FormationYear year={value.year} />
              <FormationPlace place={value.place} />
            </div>
          </div>
        );
      })}
    </section>
  );
};

export default Formation;
