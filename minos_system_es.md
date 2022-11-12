***Warning ⚠️ ️: este texto es un borrador***
# El protocolo minos

Este es un protocolo de autorización. Está diseñado pensando en la programación orientada a objetos

## Definiciones

El protocolo está basado en cuatro conceptos fundamentales.

* **Recurso**: toda entidad manipulable en un sistema.
* **Actor**: aquel que manipula las entidades.
* **Grupo**: agrupaciones de `agentes`, los agentes pueden pertenecer a más de un `grupo` o no pertenecer a ninguno.
* **Autorización**: certificado expedido a un `actor` que indica cómo puede manipular un determinado `recurso`.

### Recurso

Aquí entran las entidades de la lógica de negocio que necesiten pasar por un proceso de autorización para ser manipuladas. Cada `recurso` debe contar con los siguientes atributos:

* **Identificador único**: para poder diferenciar entre cada uno de los recursos;
* **Propietario** (opcional): identificador, nombre o firma del propietario;
* **Políticas/reglas de autorización**: cada `recurso` define cómo los `agentes` pueden manipularlo;
* **Tipo de recurso** (opcional): indica de qué tipo de entidad se trata, propuesto como una forma adicional de identificar los recursos, así como facilitar la carga de reglas de autorización para las entidades del mismo tipo.

### Actor

Entidad que es capaz de manipular los `recursos`. En un sistema típico suele ser un usuario el que manipula los recursos, sin embargo esta definición permite que cualquier entidad sea un `actor`, inclusive otro `recurso`.

Cada `actor` debe contar con los siguientes atributos:

* **Identificador único**;
* **Lista de grupos** a los que pertenece.

### Grupo

El protocolo no especifica el comportamiento de los grupos, solo se espera que contengan uno o más `agentes` ya que esta división por grupos es usada como método de autorización. Los grupos solo necesitan:

* **Identificador único**.

### Autorización

Las autorizaciones en el protocolo están pensadas tanto para ser transmitidas como almacenadas y precisan estar ligadas a un único `recurso` y a un único `actor`. 

Las `autorizaciones` necesitan los siguientes atributos:

* **Permisos**: indican las acciones que el `actor` puede realizar con el `recurso`, no están limitados al conjunto CRUD, sino que pueden representar cualquier acción.
* **identificador del `actor`**.
* **identificador del `recurso`**.
* **Tipo de recurso** (opcional).
* **Expiración**: para facilitar la transmisión las autorizaciones entre distintos sistemas informáticos y para facilitar la administración y revocación de la autorización, se obliga a establecer una fecha límite en la que aquella es válida.

### Permiso

Acciones que pueden realizar los `agentes` sobre los `recursos`. 

## Políticas de autorización

Minos propone tres tipos de autorización, por medio del propietario del `recurso`, por medio de la pertenencia a un `grupo` en específico y a la medida. Los `recursos` pueden tener un número ilimitado de políticas de autorización, por lo cual se recomienda ser específico con los permisos que se le otorga al `actor` y la duración de los mismos.

### Por propietario

Pensada para todos aquellos `recursos` que efectivamente tengan un propietario, se busca comprobar que el `actor` es propietario del `recurso`.

### Por grupo

Una lista blanca con los identificadores de los grupos a los que el actor puede pertenecer para obtener la autorización. 

### A medida

Cualquier algoritmo distinto o que complemente a los dos anteriores.

## Seguridad 

Para garantizar la seguridad de este protocolo, se debe contar con un sistema que conozca a los `recursos` y a los `agentes`; que sea capaz de transmitir las autorizaciones y de validar las autorizaciones que se le presenten. De preferencia se recomienda que se ejecute en un entorno aislado y que otros programas solo puedan interactuar con él a través de una API. Para la transmisión de `autorizaciones` se recomienda utilizar el estándar JWT o, en su defecto, guardar las `autorizaciones` en el servidor de autorización

