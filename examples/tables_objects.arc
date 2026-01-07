// Table and Object Example

// Table literal
var player = {
    name: "Hero",
    hp: 100,
    level: 5
};

var name = player.name;
var hp = player["hp"];

// Object declaration
object Enemy: {
    var hp = 50;
    var damage = 10;
    
    func attack(): {
        return damage;
    } end
} end

var enemy_hp = Enemy.hp;
var attack_damage = Enemy.attack();
