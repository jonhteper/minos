***Advertencia ⚠️ ️: este texto está en construcción***

# El sistema Minos



Sistema de autorización, diseñado para utilizar, de manera preferente, políticas de autorización escritas de manera declarativa. Minos utiliza una mezcla entre Role-Based Access Control (RBAC) y Attribute-based access control (ABAC) para generar las autorizaciones.

Se apoya en seis conceptos principales:

* **Recurso**: toda aquella parte del sistema que requiere un proceso de autorización para acceder a ella o modificarla.
* **Permiso**: acción que puede realizarse sobre un Recurso. 
* **Actor**: aquella entidad en el sistema que necesite un Permiso sobre un Recurso.
* **Grupo**: Conjunto de Actores.
* **Política**: Regla que indica las condiciones que debe de cumplir un Actor para obtener Permisos sobre un Recurso.
* **Autorización**: Certificado que contiene los Permisos concedidos a un Actor sobre un Recurso hasta una fecha en específico.

## Recursos

Minos permite que cualquier entidad en el sistema sea un Recurso, los únicos requisitos son que tenga una Política de autorización asociada (para esto se utiliza el campo `resource_type`) y cumpla con el siguiente esquema:

* **id**: identificador único del recurso. Se recomienda que sea un UUID y en ciertos casos puede ser recomendable que no sea el id real del objeto, sino un valor usado exclusivamente para el proceso de autorización (`string`).
* **resource_type**: este campo sirve como un «tipado» para reconocer el Recurso y poder decidir qué Políticas pueden ser aplicadas para generar la Autorización (`string`).
* **owner**: id del propietario del Recurso, el cual siempre debe ser un Actor. Debido a la posibilidad que los Recursos no tengan un propietario en momento de ejecución, este campo es opcional (`string`).
* **attributes**: lista de atributos del Recurso relevantes para realizar la autorización. Para evitar los caracteres innecesarios se deja fuera de las reponsabilidades del servidor Minos la definición de estos atributos. Más adelante veremos ejemplos. Opcional (`array<string>`).

 Por su diseño, inclusive los Actores pueden ser considerados Recursos, siempre que de alguna manera lleguen a Minos con el formato adecuado.

## Permisos

Al no conocer en profundidad a los recursos, Minos no puede determinar qué acciones pueden afectar a los mismos. Esto puede permitir definir nombres de permisos más allá de las operaciones CRUD. Por ejemplo, en un sistema de Blogging, un usuario podría necesitar el permiso *READ_POST*, pero no el permiso *WRITE_COMMENTARY* para poder abrir el post desde una página web, pero sí ambos permisos si quiere abrir el mismo post desde una aplicación móvil; de manera que un usuario no registrado pueda utilizar la página web, pero le sea obligatorio registrarse para utilizar la aplicación. Como puede intuirse en el ejemplo anterior las Políticas de autorización serían las mismas, solo que cada cliente decidiría qué permisos le son necesarios al Actor en cada caso.

## Actores

De la misma forma que con los Recursos Minos conoce de los Actores solo la información relevante para la autorización:

* **id**: identificador único, preferentemente un UUID (`string`).
* **groups**: lista de los grupos a los que pertenece el Actor (`array<string>`).
* **attributes**: lista de atributos[^2] (`array<string>`).

Como puede observarse no es difícil que cualquier Recurso pueda convertirse en un Actor y viceversa.

## Grupos

Los grupos solo necesitan un nombre o identificador, siendo este una cadena de texto. Si bien no es necesario, se recomienda que los Grupos tengan nombres que sean fácilmente reconocibles por humanos. Por ejemplo es preferible utilizar "writers" que "us-wr".

## Políticas de Autorización

### Definición

Son la parte central de un sistema Minos. Su esquema es el siguiente:

* resource_type: El tipo de recurso al que hace referencia (`string`).
* resource_id: Es posible enlazar una Política a un Recurso en concreto; de ser así, deben ignorarse las políticas ligadas únicamente por el `resource_type`. Opcional (`string`).
* duration: Tiempo por el cual estará autorizado el Actor, expresado en segundos (`unsigned int`).
* auth_modes: Algoritmos para construir la Autorización (`array<string>`).
* groups: Lista de grupos. Opcional, pero necesario en algunos algoritmos.(`array<string>`).
* resource_attributes: Lista de atributos en forma `key:value`[^2]. Opcional, pero necesario en algunos algoritmos (`array<string>`).
* permissions: Lista de acciones que el Actor podrá realizar en sobre el Recurso tras la autorización (`array<string>`).

[^2]: Ejemplo de lista de atributos en texto: `["status:published", "is_revised:false"]`

### Algoritmos de autorización

En Minos existen algoritmos estándar que permiten esperar resultados predecibles en tiempo de ejecución, los cuales pueden ser combinados entre sí, siguiendo algunas reglas básicas que evitan contradicciones.

* `owner`: Se concede la autorización si el Actor es propietario del recurso. Se recomienda solo utilizarlo cuando se sabe que los Recursos tienen realmente un propietario en tiempo de ejecución.
* `one_group`: Se concede la autorización si el Actor pertenece a al menos un grupo de los listados en la Política.
* `groups`: Se concede la autorización si el Actor pertenece a todos los grupos listados en la Política. No es compatible con `one_group`.
* `one_attribute`: Se concede la autorización si se encuentra en el Recurso al menos uno de los atributos descritos en la Política.
* `attributes`: Se concede la autorización si se encuentran en el Recurso todos los atributos descritos en la Política. No es compatible con `one_attribute`.
* `custom`: Se concede la autorización según una lógica que no puede ser cubierta por los anteriores algoritmos, aún combinándolos. Al escapar del estándar, no existe ninguna recomendación en su uso además que no debe romper el esquema de las Políticas ni de las llamadas estándar.[^1]

Ejemplo del modelo, escrito en Rust:

```rust
struct Policy {
    resource_type: String,
    resource_id: Option<String
    duration: u64,
    auth_mode: Vec<AuthorizationMode>,
    groups: Option<Vec<String>>,
    resource_attributes: Option<Vec<String>>,
    permissions: Vec<String>,
}

pub enum AuthorizationMode {
    Owner, 				// "owner"
    OneGroup, 			// "one_group"
    Groups, 			// "groups"
    OneAttribute,		// "one_attribute"
    Attributes,			// "attributes"
}
```

[^1]: Debido a que esta forma se aleja del estándar, por el momento no abordaremos esta modalidad de autorización; todos los siguientes ejemplos se ciñen a los algoritmos de autorización estándar.

### Representación en texto

A continuación la representación de una política de autorización en dos formatos de texto.

JSON:

```json
{
    "policies": [
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["owner"],
            "permissions": ["read", "update", "delete"]
        }
    ]
}
```

TOML:

```toml
[[policies]]
resource_type = "blog_post"
duration = 2
auth_mode = ["owner"]
permissions = ["read", "update", "delete"]
```

Si bien, como puede apreciarse, el formarto TOML puede tanto más fácil de leer como más ligero (y por ello recomendamos utilizarlo como formato de guardado en la capa de persistencia) en adelante utilizaremos el estándar JSON para realizar los ejemplos.

## Autorizaciones

Minos plantea las autorizaciones como objetos que pueden ser transmitidos y almacenados para su posterior uso. Por lo anterior, deben tener un formato como el siguiente:

* **id**: identificador único, debe ser un UUID (`string`).
* **permissions**: lista de Permisos concedidos al Actor (`array<string>`).
* **actor_id**: identificador único del Actor a quien se le ha concedido la Autorización (`string`).
* **resource_id**: identificador único del Recurso sobre el que se conceden los Permisos (`string`).
* **resource_type**: tipo de recurso sobre el que se conceden los Permisos (`string`).
* **expiration**: fecha Unix en que la Autorización deja de ser válida (`int`).

Con los datos anteriores, un cliente podría volver a validar la autorización y de esa forma ahorrarse llamar nuevamente al servidor Minos. Por supuesto que se hace necesario un sistema que garantice que el Actor no modifica la Autorización a su gusto, y nosotros recomendamos utilizar para este fin los Json Web Tokens.

## Llamadas estándar

### get_authorization

`get_authorization(ActorRequest, ResourceRequest) : Authorization`

### user_has_permissons

`user_has_permissons(ActorRequest, ResourceRequest, Permissions) : boolean`

### authorization_is_valid

`authorization_is_valid(Authorization) : boolean`

## Llamadas no estándar

* send_policies
* autorization_by_policy

## Funcionamiento de un Sistema Minos

### Requests

ActorRequest

```rust
struct ActorRequest {
    id: String,
    groups: Option<Vec<String>>
}
    
```

ResourceRequest 

```rust
struct ResourceRequest  {
	id: Option<String>,
    resource_type: String,
    owner: Option<String>,
    attributes: Option<Vec<String>>,
}
```

Permissions

```rust
type Permissions = Vec<String>;
```

Authorization

```rust
struct Authorization {
    id: String,
    permissions: Permissions,
    actor_id: String,
    resource_id: String,
    resource_type: String,
    expiration: i64,
}
```

### Ejemplos de comportamiento

Tomemos como ejemplo un Servidor de Blogging que envía a un Servidor Minos las siguientes Políticas de Autorización.

```json
{
    "policies": [
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["owner"],
            "permissions": ["read", "update", "delete"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["one_group"],
            "groups":["readers", "admins"],
            "permissions": ["read"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["groups"],
            "groups":["admins", "writers"],
            "permissions": ["delete"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["one_attribute"],
            "resource_attributes": ["status:published"],
            "permissions": ["read"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["owner attributes"],
            "resource_attributes": ["status:writed", "is_revised:true"],
            "permissions": ["publish"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["one_group one_attribute"],
            "groups":["admins", "writers"],
            "resource_attributes": ["status:archived"],
            "permissions": ["re_publish"]
        },
        {
            "resource_type": "blog_post",
            "duration": 2,
            "auth_mode": ["groups one_attribute"],
            "groups":["admins"],
            "resource_attributes": ["status:published"],
            "permissions": ["archive"]
        }
    ]
}
```

#### Ejemplo 1

Algún tiempo después un Actor realiza el llamado `get_authorization` y el servidor Minos obtiene los siguientes datos:

```json
{
    "actor":  {
        "id": "actor.example.id",
        "groups": ["admins", "writers"]
    },
    "resource": {
        "id": "blogpost.example.id",
        "resource_type": "blog_post",
        "owner": "actor.example.id",
        "attributes": ["status:writed"]
    }
}
```

Por lo tanto la autorización generada será:

```json
{
    "authorization": {
        "id": "auth.example.01.id",
        "permissions": ["read", "update", "delete"],
        "actor_id": "actor.example.id",
        "resource_id": "blogpost.example.id",
        "resource_type:": "blog_post",
        "expiration": 1676170760
    }
}
```

Esto porque si recorremos todas las Políticas el Actor obtiene esa lista de permisos, de la siguiente forma:

* El Actor es propietario del Recurso, por lo tanto obtiene los permisos `read`, `update` y `delete`.
* El Actor pertenece al grupo `admins`, por lo tanto obtiene el permiso `read`, sin embargo como ya lo tiene por la anterior Política, no se altera la lista de permisos.
* El actor pertenece a los dos grupos indicados, por lo tanto obtiene el permiso `delete`, y sucede como en la Política anterior: la lista no se altera.
* El Recurso no tiene el atributo `published`, por lo tanto no se obtienen permisos y la lista no se altera.
* El Actor es propietario del Recurso, pero el Recurso no tiene el atributo `revised`, por lo tanto no se obtienen permisos.
* El Actor pertenece a al menos un grupo de los indicados, pero el Recurso no tiene el atributo `archived`, por tanto no obtiene más permisos.
* El Actor pertenece a todos los grupos listados, pero el Recurso no tiene el atributo `published`, así que no se obtienen permisos.

La lista queda entonces con los permisos `read`, `update` y `delete`.

Es importante señalar que aunque el Actor no cumpla la mayoría de las Políticas al tener permisos disponibles no se retorna ningún error.

#### Ejemplo 2

Supongamos que otro Actor realiza la misma solicitud y el servidor Minos obtiene los siguientes datos:

```json
{
    "actor":  {
        "id": "guest.actor.id",
        "groups": []
    },
    "resource": {
        "id": "blogpost.example.id",
        "resource_type": "blog_post",
        "owner": "actor.example.id",
        "attributes": ["status:writed"]
    }
}
```

En ese caso, no se genera ninguna autorización, dado que el Actor no tiene ningún Permiso sobre el Recurso y el Servidor Minos debería retornar un mensaje de error.

#### Ejemplo 3

Supongamos que el mismo Actor realiza la anterior solicitud, sin embargo el post ya ha sido publicado y el servidor Minos obtiene los datos:

```json
{
    "actor":  {
        "id": "guest.actor.id",
        "groups": []
    },
    "resource": {
        "id": "blogpost.example.id",
        "resource_type": "blog_post",
        "owner": "actor.example.id",
        "attributes": ["status:published"]
    }
}
```

Por lo tanto la Autorización generada será:

```json
{
    "authorization": {
        "id": "auth.example.04.id",
        "permissions": ["read"],
        "actor_id": "guest.actor.id",
        "resource_id": "blogpost.example.id",
        "resource_type:": "blog_post",
        "expiration": 1676170760
    }
}
```



## Implementar un Sistema Minos

Como puede entenderse con los conceptos anteriores, no existe una única forma válida de crear un Sistema Minos. Para su implementación...

### Backend Manages Authorizations (BMA)

![image-20230127171729684](/home/jonhteper/.config/Typora/typora-user-images/image-20230127171729684.png)

En el diagrama superior...

### Frontend Manages Authorizations (FMA)

![image-20230127202848555](/home/jonhteper/.config/Typora/typora-user-images/image-20230127202848555.png)

### Minos Manages Authorizations (MMA)

![image-20230127202413373](/home/jonhteper/.config/Typora/typora-user-images/image-20230127202413373.png)

### Minos Implements Actors Log

![image-20230208113456718](/home/jonhteper/.config/Typora/typora-user-images/image-20230208113456718.png)

