package com.cheezgi.petsim;

import com.cheezgi.petsim.Pet; //for Pet class
import java.util.Scanner; //for user input

public class PetSimulator {
	//no arguments used in this program
	public static void main(String[] args) {
		
		//to gather user input
		Scanner scan = new Scanner(System.in);
		
		System.out.println("Welcome to Pet Simulator!");
		
		System.out.print("What would you like to name your pet? ");
		String name = scan.nextLine(); //scan user input and assign name to this value
		
		System.out.print("What type of pet would you like? ");
		String type = scan.nextLine(); //scan user input and assign type to this value
		
		System.out.println(name + " the " + type + "! Nice choice.");
		System.out.println("If you need help at any time, please type help.");
		
		//initialize pet object
		Pet pet = new Pet(false, 100, 45, 45, 1, 1, name, type);
		
		String input; //for user input
		int cmdCounter = 0; //amount of commands run by the user
		int jobCounter = 0; //counter used to gain money
		
		//while the pet is not dead
		while(!pet.dead){
			System.out.print("> ");
			
			//gather input from the user and assign the value to input
			input = scan.nextLine();
			
			//check value of lowercased input against cases
			switch(input.toLowerCase()){
			case "eat":
				System.out.print("Amount (number): ");
				input = scan.nextLine();
				
				//integer's parseInt method: String -> integer | amount
				int amount = Integer.parseInt(input);
				
				//pet's eat method: integer -> nothing
				pet.eat(amount);
				
				cmdCounter++;
				break;
			case "play":
				System.out.print("Time (number): ");
				input = scan.nextLine();
				
				int time = Integer.parseInt(input);
				
				pet.play(time);
				
				cmdCounter++;
				break;
			case "pet":
				System.out.print("Times (number): ");
				input = scan.nextLine();
				
				int times = Integer.parseInt(input);
				
				pet.pet(times);
				
				cmdCounter++;
				break;
			case "bathe":
				pet.bathe();
				cmdCounter++;
				break;
			case "buy":
				System.out.print("What to buy: ");
				String tmp = scan.nextLine();
				
				System.out.print("How much (number): ");
				input = scan.nextLine();
				
				pet.buy(tmp, Integer.parseInt(input));
				
				cmdCounter++;
				break;
			case "hurt":
				System.out.print("Why would you hurt " + pet.name + "?");
				pet.hurt();
				
				cmdCounter++;
				break;
			case "kill":
				System.out.println("Thanks for playing..?");
				pet.dead = true;
				break;
			case "stats":
				pet.stats();
				break;
			case "exit":
				System.out.print("You will lose any progress made. Are you sure?");
				input = scan.nextLine();
				
				if(input.equals("y")){
					System.out.println("Thanks for playing!");
					pet.dead = true;
				}
				else if(input.equals("n")){
					break;
				}
				else{
					System.out.println("Input not recognized, aborting.");
				}
				break;
			case "help":
				System.out.println("This feature is coming soon. For now, you");
				System.out.println("can check the source code. Thanks! -cheezgi");
				break;
			default:
				System.out.println("Unknown command. Please try \'help\' if you need help.");
			}
			
			jobCounter++;
			
			if(jobCounter == 5){
				System.out.println("The \"work\" week is over. Your bitcoin miner has earned ¤15");
				pet.inventory[1] += 15;
				
				jobCounter = 0;
			}
			
			if(cmdCounter >= 3){
				if(pet.hunger == 0.0){
					System.out.println(pet.name + " is hungry and loses some health.");
					pet.hurt();
				}
				
				if(pet.hunger != 0.0){
					pet.hunger -= 0.5;
				}
				
				cmdCounter = 0;
			}
			
			input = "";
		}
		
		scan.close();
		System.exit(0); //make it official
	}

}
