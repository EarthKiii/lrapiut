/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class NotesService {
  /**
   * Creates a NotesService without using a LRUser.
   * <div class="warning">
   *    This method is not recommended as using notes's credentials will soon allow you to get gpu's credentials.
   *    It is better to use the LRUser method for future proof code if you plan to use GpuService later on.
   * </div>
   *
   * # Examples
   * ```
   * use lrapiut::notes::NotesService;
   *
   * let notesService = NotesService::new("username".to_string(), "password".to_string());
   * ```
   */
  constructor(username: string, password: string)
  /** Returns the `semestreEtudiant` endpoint. */
  semestreEtudiant(): Promise<any>
  /** Returns the `dataPremièreConnexion` endpoint. */
  dataPremiereConnexion(): Promise<any>
  /**
   * Returns the `relevéEtudiant` endpoint.
   *
   * # Arguments
   *
   * * `semestre` - either the semester_id or the formsemester_id.
   */
  releveEtudiant(semestre: number): Promise<any>
  /** Returns the `deleteStudentPic` endpoint. */
  deleteStudentPic(): Promise<any>
  /** Returns the `getStudentPic` endpoint. */
  getStudentPic(): Promise<any>
  /**
   * Returns the `setStudentPic` endpoint.
   * ## TODO
   */
  setStudentPic(): Promise<any>
  /** Returns the `donnéesAuthentification` endpoint. */
  donneesAuthentification(): Promise<any>
  /** Returns the `listeNotes` endpoint. */
  listeNotes(eval: number): Promise<any>
}
export type LRUser = LrUser
/** Represents an IUT La Rochelle student. */
export declare class LrUser {
  /**
   * Creates a new LRUser.
   *
   * # Arguments
   *
   * * `username` - The student's username.
   * * `password` - The student's password.
   *
   * # Examples
   * ```
   * use lrapiut::LRUser;
   *
   * let lrUser = LRUser::new("username".to_string(), "password".to_string());
   * ```
   */
  constructor(username: string, password: string)
  /**
   * Sets currents student's credentials.
   *
   * # Arguments
   *
   * * `username` - The new student's username.
   * * `password` - The new student's password.
   *
   * # Examples
   * ```
   * use lrapiut::LRUser;
   *
   * let lrUser = LRUser::new("username".to_string(), "password".to_string());
   * lrUser.set_credentials("new_username".to_string(), "new_password".to_string());
   * ```
   */
  setCredentials(username: string, password: string): void
  /**
   * Gets currents student's credentials.
   *
   * <div class="warning">
   *     This method is meant to be used in javascript only.
   * </div>
   *
   * # Examples
   * ```
   * use lrapiut::LRUser;
   *
   * let lrUser = LRUser::new("username".to_string(), "password".to_string());
   * lrUser.get_credentials(/* `env` is injected by NAPI-RS */);
   * ```
   *
   * # Returns
   * ```
   * {"username": "username", "password": "password"}
   * ```
   */
  getCredentials(): object
  /**
   * Gets the notes service, it is used to access notes's endpoints.
   *
   * <div class="warning">
   *     This method does not need to be called with parenthesis in javascript as it is binded as a getter.
   * </div>
   *
   * # Examples
   * ```
   * use lrapiut::LRUser;
   *
   * let lrUser = LRUser::new("username".to_string(), "password".to_string());
   * let notesService = lrUser.notes();
   * ```
   */
  get notes(): NotesService
}
