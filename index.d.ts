/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class NotesService {
  constructor(username: string, password: string)
  semestreEtudiant(): Promise<any>
  dataPremiereConnexion(): Promise<any>
  releveEtudiant(semestre: number): Promise<any>
  deleteStudentPic(): Promise<any>
  getStudentPic(): Promise<any>
  setStudentPic(): Promise<any>
  donneesAuthentification(): Promise<any>
}
export type LRUser = LrUser
export class LrUser {
  constructor(username: string, password: string)
  setCredentials(username: string, password: string): void
  getCredentials(): object
  get notes(): NotesService
}
