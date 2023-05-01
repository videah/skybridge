import 'package:json_annotation/json_annotation.dart';
import 'package:sky_bridge/models/oauth/oauth_authorize_params.dart';

part 'sign_in_form.g.dart';

/// A form that is used to sign in and get an access token.
@JsonSerializable()
class SignInForm {
  /// Creates an instance of [SignInForm].
  SignInForm({
    required this.stuff,
    required this.password,
  });

  /// Converts JSON into an [SignInForm] instance.
  factory SignInForm.fromJson(Map<String, dynamic> json) =>
      _$SignInFormFromJson(json);

  /// Converts an [SignInForm] instance into JSON.
  Map<String, dynamic> toJson() => _$SignInFormToJson(this);

  /// A signed and stuffed [OAuthAuthorizeParams] string.
  final String stuff;

  /// The password entered by the user.
  /// Must match SKYBRIDGE_AUTH_PASSWORD to be valid.
  final String password;
}