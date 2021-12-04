use crate::{source::registration_application::*, traits::Crud};
use diesel::{insert_into, result::Error, PgConnection, QueryDsl, RunQueryDsl};

impl Crud for RegistrationApplication {
  type Form = RegistrationApplicationForm;
  type IdType = i32;
  fn create(conn: &PgConnection, form: &Self::Form) -> Result<Self, Error> {
    use crate::schema::registration_application::dsl::*;
    insert_into(registration_application)
      .values(form)
      .get_result::<Self>(conn)
  }

  fn read(conn: &PgConnection, id_: Self::IdType) -> Result<Self, Error> {
    use crate::schema::registration_application::dsl::*;
    registration_application.find(id_).first::<Self>(conn)
  }

  fn update(conn: &PgConnection, id_: Self::IdType, form: &Self::Form) -> Result<Self, Error> {
    use crate::schema::registration_application::dsl::*;
    diesel::update(registration_application.find(id_))
      .set(form)
      .get_result::<Self>(conn)
  }

  fn delete(conn: &PgConnection, id_: Self::IdType) -> Result<usize, Error> {
    use crate::schema::registration_application::dsl::*;
    diesel::delete(registration_application.find(id_)).execute(conn)
  }
}
