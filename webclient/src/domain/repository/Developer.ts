export class Developer {
  constructor(
    public id: bigint,
    public updated_at: Date,
    public user_name: string,
    public full_name: string,
    public email: string,
    public primary_lang_id: bigint,
    public secondary_lang_id?: bigint
  ) {}
}
