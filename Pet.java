package com.cheezgi.petsim;

public class Pet {
	boolean dead = false;
	int health;
	double happiness;
	double hunger;
	int age;
	int cleanliness;
	String name;
	String type;
	//food, money
	int[] inventory = {15, 25};
	
	Pet(boolean dead, int health, double happiness, double hunger, int age,
			int cleanliness, String name, String type){
		
		this.dead = dead;
		this.health = health;
		this.happiness = happiness;
		this.hunger = hunger;
		this.age = age;
		this.cleanliness = cleanliness;
		this.name = name;
		this.type = type;
		
	}
	
	void eat(int amount){
		if(hunger <= 45){
			System.out.println("You feed " + name + ".");
			
			hunger += amount;
			inventory[0] -= amount;
			
			return;
		}
		else{
			System.out.println("Your pet is already full!");
			return;
		}
	}
	
	void play(int time){
		System.out.println("You play with " + name + ". " + name + " gains some happiness.");
		happiness += 1;
	}
	
	void pet(int times){
		System.out.println("You pet " + name + ". " + name + " gains some happiness.");
		happiness += 0.2;
	}
	
	void bathe(){
		System.out.println("You bathe " + name + ". " + name + " becomes cleaner.");
		cleanliness--;
	}
	
	void buy(String item, int quantity){
		switch(item){
		case "food":
			System.out.println("You buy" + quantity + " food and lose ¤" + quantity + ".");
			
			inventory[0] += quantity;
			inventory[1] -= quantity;
			break;
		//more later
		}
	}
	
	void stats(){
		System.out.println("dead: " + dead);
		System.out.println("health: " + health + "; max: 100");
		System.out.println("happiness: " + happiness);
		System.out.println("hunger: " + hunger + "; max: 10");
		System.out.println("age: " + age);
		System.out.println("cleanliness: " + cleanliness);
		System.out.println("name: " + name);
		System.out.println("type: " + type);
		System.out.println("money: ¤" + inventory[1]);
		System.out.println("food: " + inventory[0]);
	}
	void hurt(){}
	
}
